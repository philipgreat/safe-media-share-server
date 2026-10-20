import { useCallback, useEffect, useRef, useState } from 'react'
import { Link } from 'react-router-dom'
import { api, copyText, formatSize, joinExternal, getAdminKey } from '../api.js'

function formatBytes(n) {
  return formatSize(n)
}

function uploadOne(file, relPath, name, onProgress) {
  return new Promise((resolve, reject) => {
    const form = new FormData()
    form.append('file', file, file.name)
    form.append('file_path', relPath)
    if (name) form.append('name', name)

    const xhr = new XMLHttpRequest()
    xhr.open('POST', '/admin/api/media')
    xhr.setRequestHeader('X-Admin-Key', getAdminKey())

    xhr.upload.onprogress = (e) => {
      if (e.lengthComputable && onProgress) {
        onProgress(e.loaded, e.total)
      }
    }
    xhr.onload = () => {
      let body = {}
      try {
        body = JSON.parse(xhr.responseText || '{}')
      } catch (_) {}
      if (xhr.status >= 200 && xhr.status < 300 && body.status !== 'NO') {
        resolve(body.data?.[0] || {})
      } else {
        reject(new Error(body.message || `HTTP ${xhr.status}`))
      }
    }
    xhr.onerror = () => reject(new Error('network error'))
    xhr.send(form)
  })
}

export default function AdminMedia({ settings }) {
  const [items, setItems] = useState([])
  const [error, setError] = useState('')
  const [toast, setToast] = useState('')
  const [queue, setQueue] = useState([]) // {id, file, relPath, loaded, total, status, message, assetId}
  const [uploading, setUploading] = useState(false)
  const fileInputRef = useRef(null)
  const dirInputRef = useRef(null)

  const load = useCallback(async () => {
    try {
      const { res, body } = await api('/admin/api/media')
      if (!res.ok) {
        setError(body.message || 'Load failed')
        return
      }
      setItems(body.data || [])
      setError('')
    } catch (e) {
      setError(String(e))
    }
  }, [])

  useEffect(() => {
    load()
  }, [load])

  function enqueue(files, fromDirectory) {
    const list = []
    for (const f of files) {
      const rel =
        fromDirectory && f.webkitRelativePath
          ? f.webkitRelativePath
          : f.name
      list.push({
        id: `${Date.now()}-${Math.random().toString(36).slice(2)}`,
        file: f,
        relPath: rel,
        loaded: 0,
        total: f.size,
        status: 'pending',
        message: '',
      })
    }
    setQueue((q) => [...q, ...list])
    return list
  }

  async function runUpload() {
    if (uploading) return
    setUploading(true)
    // work off a snapshot
    let pending = []
    setQueue((q) => {
      pending = q.map((x) => ({ ...x }))
      return q
    })
    await new Promise((r) => setTimeout(r, 0))

    for (const item of pending) {
      if (item.status === 'done') continue
      setQueue((q) =>
        q.map((x) =>
          x.id === item.id ? { ...x, status: 'uploading', loaded: 0 } : x,
        ),
      )
      try {
        const result = await uploadOne(
          item.file,
          item.relPath,
          '',
          (loaded, total) => {
            setQueue((q) =>
              q.map((x) => (x.id === item.id ? { ...x, loaded, total } : x)),
            )
          },
        )
        // Upload response carries ref via viewUrl; keep numeric id for display only.
        const refMatch = String(result.viewUrl || '').match(/\/view\/([^/?#]+)/)
        const assetRef = refMatch ? decodeURIComponent(refMatch[1]) : ''
        setQueue((q) =>
          q.map((x) =>
            x.id === item.id
              ? {
                  ...x,
                  status: 'done',
                  loaded: x.total,
                  assetId: result.mediaAssetId,
                  assetRef,
                  message: `asset #${result.mediaAssetId}`,
                }
              : x,
          ),
        )
      } catch (e) {
        setQueue((q) =>
          q.map((x) =>
            x.id === item.id
              ? { ...x, status: 'error', message: String(e.message || e) }
              : x,
          ),
        )
      }
    }
    setUploading(false)
    await load()
    setToast('Upload finished')
    setTimeout(() => setToast(''), 1800)
  }

  function clearDone() {
    setQueue((q) => q.filter((x) => x.status !== 'done'))
  }

  function onCopy(text, label) {
    copyText(text).then(() => {
      setToast(`Copied ${label}`)
      setTimeout(() => setToast(''), 1500)
    })
  }

  async function onGenerate(id) {
    const { res, body } = await api(`/admin/api/invites?asset_id=${id}&note=admin-generated`, {
      method: 'POST',
    })
    if (res.ok && body.data?.[0]) {
      await copyText(body.data[0].inviteToken)
      setToast('Invite created and copied')
      setTimeout(() => setToast(''), 1500)
      await load()
    } else {
      setToast(body.message || 'Failed')
      setTimeout(() => setToast(''), 2000)
    }
  }

  const base = settings?.externalBaseUrl || ''
  const totalBytes = queue.reduce((s, x) => s + (x.total || 0), 0)
  const loadedBytes = queue.reduce((s, x) => s + (x.loaded || 0), 0)
  const overall = totalBytes > 0 ? Math.floor((loadedBytes / totalBytes) * 100) : 0

  return (
    <>
      <header className="page-head">
        <h2>Media</h2>
        <p className="muted">Resources, invites, and uploads</p>
      </header>

      <section className="settings-card" style={{ maxWidth: 'none', marginBottom: 28 }}>
        <h3>Upload</h3>
        <p className="muted small" style={{ margin: 0 }}>
          Add single files or a whole directory. Relative folder paths are preserved under the media root.
        </p>
        <div className="row" style={{ marginTop: 8 }}>
          <button
            type="button"
            className="btn"
            onClick={() => fileInputRef.current?.click()}
          >
            Select files
          </button>
          <button
            type="button"
            className="btn secondary"
            onClick={() => dirInputRef.current?.click()}
          >
            Select folder
          </button>
          <button
            type="button"
            className="btn"
            disabled={uploading || !queue.some((x) => x.status === 'pending' || x.status === 'error')}
            onClick={runUpload}
          >
            {uploading ? `Uploading ${overall}%` : 'Start upload'}
          </button>
          <button type="button" className="btn secondary" onClick={clearDone}>
            Clear done
          </button>
        </div>

        <input
          ref={fileInputRef}
          type="file"
          multiple
          hidden
          onChange={(e) => {
            enqueue(Array.from(e.target.files || []), false)
            e.target.value = ''
          }}
        />
        <input
          ref={dirInputRef}
          type="file"
          multiple
          // @ts-ignore - directory selection
          webkitdirectory=""
          directory=""
          hidden
          onChange={(e) => {
            enqueue(Array.from(e.target.files || []), true)
            e.target.value = ''
          }}
        />

        {queue.length > 0 && (
          <div style={{ marginTop: 12 }}>
            <div className="field-label">
              Queue · {queue.length} file(s) · {formatBytes(loadedBytes)} / {formatBytes(totalBytes)}
              {totalBytes > 0 ? ` · ${overall}%` : ''}
            </div>
            <div className="media-list">
              {queue.map((x) => {
                const pct = x.total > 0 ? Math.floor((x.loaded / x.total) * 100) : 0
                return (
                  <div className="media-row" key={x.id} style={{ gridTemplateColumns: '1fr 120px' }}>
                    <div>
                      <div className="media-name code">{x.relPath}</div>
                      <div className="media-meta">
                        <span>{formatBytes(x.total)}</span>
                        <span className="sep">/</span>
                        <span>{x.status}</span>
                        {x.message ? (
                          <>
                            <span className="sep">/</span>
                            <span>{x.message}</span>
                          </>
                        ) : null}
                        {x.status === 'uploading' && (
                          <>
                            <span className="sep">/</span>
                            <span>{pct}%</span>
                          </>
                        )}
                      </div>
                      <div
                        style={{
                          marginTop: 6,
                          height: 2,
                          background: 'var(--line)',
                          maxWidth: 420,
                        }}
                      >
                        <div
                          style={{
                            height: 2,
                            width: `${x.status === 'done' ? 100 : pct}%`,
                            background: 'var(--fg)',
                          }}
                        />
                      </div>
                    </div>
                    <div className="media-actions" style={{ justifyContent: 'flex-end' }}>
                      {x.status === 'done' && x.assetRef ? (
                        <Link className="copy-btn" to={`/view/${encodeURIComponent(x.assetRef)}`}>
                          Open
                        </Link>
                      ) : (
                        <span className="tag">{x.status}</span>
                      )}
                    </div>
                  </div>
                )
              })}
            </div>
          </div>
        )}
      </section>

      {error && <div className="notice err">{error}</div>}

      <div className="table-wrap">
        <table className="admin-table">
          <thead>
            <tr>
              <th>ID</th>
              <th>Name</th>
              <th>Path</th>
              <th>Size</th>
              <th>Invite token</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {items.map((it) => {
              const token = it.inviteToken || ''
              const inviteUrl =
                it.inviteUrl || (token ? joinExternal(base, `/invite/${token}`) : '')
              const viewUrl = it.viewUrl || joinExternal(base, `/view/${encodeURIComponent(it.ref || '')}`)
              const downloadUrl =
                it.downloadUrl || joinExternal(base, `/media/${encodeURIComponent(it.ref || '')}/download`)
              return (
                <tr key={it.id}>
                  <td className="tag">{it.id}</td>
                  <td>
                    <Link to={`/view/${encodeURIComponent(it.ref)}`}>{it.displayName || it.filePath}</Link>
                  </td>
                  <td className="code">{it.filePath}</td>
                  <td>{formatBytes(it.fileSize)}</td>
                  <td className="code">{token || <span className="faint">—</span>}</td>
                  <td>
                    <a className="copy-btn" href={viewUrl} target="_blank" rel="noreferrer">
                      Open
                    </a>
                    <a className="copy-btn" href={downloadUrl}>
                      Download
                    </a>
                    {token ? (
                      <>
                        <button type="button" className="copy-btn" onClick={() => onCopy(token, 'token')}>
                          Copy token
                        </button>
                        <button type="button" className="copy-btn" onClick={() => onCopy(inviteUrl, 'link')}>
                          Copy link
                        </button>
                      </>
                    ) : (
                      <button type="button" className="copy-btn" onClick={() => onGenerate(it.id)}>
                        Create invite
                      </button>
                    )}
                  </td>
                </tr>
              )
            })}
          </tbody>
        </table>
        {!error && items.length === 0 && <p className="empty">No media.</p>}
      </div>
      {toast && <div className="toast">{toast}</div>}
    </>
  )
}
