import { useEffect, useMemo, useState } from 'react'
import { Link, useNavigate, useSearchParams } from 'react-router-dom'
import { api, formatSize, kindLabel, kindOf, normalizeToken } from '../api.js'

export default function UserHome() {
  const [items, setItems] = useState([])
  const [token, setToken] = useState('')
  const [msg, setMsg] = useState(null)
  const [loading, setLoading] = useState(true)
  const [submitting, setSubmitting] = useState(false)
  const navigate = useNavigate()
  const [params] = useSearchParams()

  useEffect(() => {
    let cancelled = false
    ;(async () => {
      try {
        const { res, body } = await api('/api/media')
        if (!cancelled && res.ok) setItems(body.data || [])
      } catch (e) {
        if (!cancelled) setMsg({ kind: 'err', text: `加载失败：${e}` })
      } finally {
        if (!cancelled) setLoading(false)
      }
    })()
    return () => {
      cancelled = true
    }
  }, [])

  useEffect(() => {
    const code = params.get('code') || params.get('invite')
    if (code) {
      setToken(code)
      setMsg({ kind: 'warn', text: '已填入邀请码，请提交。' })
    }
    if (params.get('expired') === '1') {
      setMsg({ kind: 'warn', text: '第二次提交：当前 IP 已从白名单移除。' })
    }
    if (params.get('spent') === '1') {
      setMsg({ kind: 'err', text: '邀请码已失效，不能再使用。' })
    }
    if (params.get('invalid') === '1') {
      setMsg({ kind: 'err', text: '邀请码无效或不存在。' })
    }
  }, [params])

  async function onSubmit(ev) {
    ev.preventDefault()
    const t = normalizeToken(token)
    if (!t) {
      setMsg({ kind: 'err', text: '请输入邀请码' })
      return
    }
    setSubmitting(true)
    setMsg({ kind: 'warn', text: '正在验证邀请码…' })
    try {
      const res = await fetch(`/invite/${encodeURIComponent(t)}`, {
        headers: { Accept: 'application/json' },
      })
      const body = await res.json().catch(() => ({}))
      const item = body?.data?.[0]

      if (res.status === 410 || /spent|失效|410/i.test(body.message || '')) {
        setMsg({ kind: 'err', text: '邀请码已失效，无法再使用。' })
        return
      }
      if (!res.ok || body.status === 'NO') {
        setMsg({ kind: 'err', text: body.message || `邀请无效（HTTP ${res.status}）` })
        return
      }
      if (item?.action === 'join_whitelist') {
        setMsg({
          kind: 'ok',
          text: `已加入白名单（${item.clientIp || ''}），进入媒体…`,
        })
        setTimeout(() => navigate(`/view/${encodeURIComponent(item.mediaRef || '')}`), 300)
        return
      }
      if (item?.action === 'expire_whitelist') {
        setMsg({
          kind: 'warn',
          text: `已将 ${item.clientIp || ''} 从白名单移除。邀请码不可再用于加入。`,
        })
        return
      }
      setMsg({ kind: 'warn', text: body.message || '未知响应' })
    } catch (e) {
      setMsg({ kind: 'err', text: `请求失败：${e}` })
    } finally {
      setSubmitting(false)
    }
  }

  const list = useMemo(() => items, [items])

  return (
    <>
      <header className="top">
        <h1>
          <Link to="/">Media Guard</Link>
        </h1>
        <p className="sub">Invite code → media</p>
      </header>

      <main className="wrap">
        <section>
          <h2 className="hero-h2" style={{ margin: '0 0 4px', fontSize: 18, fontWeight: 600 }}>
            Enter invite code
          </h2>
          <p className="muted small" style={{ margin: 0 }}>
            Paste a code or full /invite/ link.
          </p>
          <form className="invite-form" onSubmit={onSubmit}>
            <input
              value={token}
              onChange={(e) => setToken(e.target.value)}
              placeholder="INV-…"
              spellCheck={false}
              required
            />
            <button className="btn" type="submit" disabled={submitting}>
              {submitting ? 'Working…' : 'Open media'}
            </button>
          </form>
          {msg && <div className={`notice ${msg.kind}`}>{msg.text}</div>}
          <ol className="help">
            <li>First submit — this IP joins the whitelist (~7 days).</li>
            <li>Second submit — this IP is removed from the whitelist.</li>
            <li>After that the code is spent and cannot be reused.</li>
          </ol>
        </section>

        <section>
          <h2 className="section-title">Library</h2>
          {loading ? (
            <p className="muted">Loading…</p>
          ) : list.length === 0 ? (
            <p className="empty">No media registered.</p>
          ) : (
            <div className="media-list">
              {list.map((it) => {
                const kind = kindOf(it.contentType, it.filePath)
                return (
                  <div className="media-row" key={it.ref}>
                    <div>
                      <div className="media-name">
                        <Link to={`/view/${encodeURIComponent(it.ref)}`}>{it.displayName || it.filePath}</Link>
                      </div>
                      <div className="media-meta">
                        <span className="kind">{kindLabel(kind)}</span>
                        <span>{it.contentType}</span>
                        <span className="sep">/</span>
                        <span>{formatSize(it.fileSize)}</span>
                        <span className="sep">/</span>
                        <span className="code">{it.filePath}</span>
                      </div>
                    </div>
                    <div className="media-actions">
                      <Link className="btn secondary" to={`/view/${encodeURIComponent(it.ref)}`}>
                        Open
                      </Link>
                      <a className="btn secondary" href={`/media/${encodeURIComponent(it.ref)}/download`}>
                        Download
                      </a>
                    </div>
                  </div>
                )
              })}
            </div>
          )}
        </section>
      </main>
    </>
  )
}
