import { useEffect, useState } from 'react'
import { Link, useParams } from 'react-router-dom'
import { api, formatSize, kindLabel, kindOf } from '../api.js'

export default function ViewPage() {
  const { ref } = useParams()
  const [item, setItem] = useState(null)
  const [error, setError] = useState('')
  const [authorized, setAuthorized] = useState(false)

  useEffect(() => {
    let cancelled = false
    ;(async () => {
      try {
        const { res, body } = await api(`/api/media/${encodeURIComponent(ref)}`)
        const row = body?.data?.[0]
        if (!res.ok || !row) {
          if (!cancelled) setError(body?.message || `Cannot load media ${ref || ''}`)
          return
        }
        if (!cancelled) {
          setItem(row)
          setAuthorized(!!row.authorized)
        }
      } catch (e) {
        if (!cancelled) setError(String(e))
      }
    })()
    return () => {
      cancelled = true
    }
  }, [ref])

  return (
    <>
      <header className="top">
        <h1>
          <Link to="/">Media Guard</Link>
        </h1>
        <p className="sub">
          <Link to="/">← Library</Link>
        </p>
      </header>

      <main className="wrap">
        {error ? (
          <div className="notice err">{error}</div>
        ) : !item ? (
          <p className="muted">Loading…</p>
        ) : (
          <>
            <section>
              <div className="filename">{item.displayName || item.filePath}</div>
              <div className="meta-line">
                <span className="kind">{kindLabel(kindOf(item.contentType, item.filePath))}</span>
                <span>{item.contentType}</span>
                <span className="sep">/</span>
                <span>{formatSize(item.fileSize)}</span>
                <span className="sep">/</span>
                <span className="code">{item.filePath}</span>
                <span className="sep">/</span>
                <span>{authorized ? 'Authorized' : 'Not authorized'}</span>
              </div>
            </section>

            <section>
              {!authorized && (
                <div className="notice warn">
                  This IP is not on the whitelist. Use an invite code first.
                </div>
              )}
              {kindOf(item.contentType, item.filePath) === 'video' && (
                <video controls preload="metadata" src={`/media/${encodeURIComponent(ref)}`} />
              )}
              {kindOf(item.contentType, item.filePath) === 'audio' && (
                <audio controls preload="metadata" src={`/media/${encodeURIComponent(ref)}`} />
              )}
              {kindOf(item.contentType, item.filePath) === 'image' && (
                <img src={`/media/${encodeURIComponent(ref)}`} alt={item.displayName || ''} />
              )}
              {kindOf(item.contentType, item.filePath) === 'doc' &&
                (item.contentType || '').includes('pdf') && (
                  <iframe title="pdf" src={`/media/${encodeURIComponent(ref)}`} />
                )}
              {kindOf(item.contentType, item.filePath) === 'archive' && (
                <div className="notice">This archive can be downloaded.</div>
              )}

              <div className="row">
                <a className="btn" href={`/media/${encodeURIComponent(ref)}/download`}>
                  Download
                </a>
                <a
                  className="btn secondary"
                  href={`/media/${encodeURIComponent(ref)}`}
                  target="_blank"
                  rel="noreferrer"
                >
                  Open raw
                </a>
                <Link className="btn secondary" to="/">
                  Back
                </Link>
              </div>
            </section>
          </>
        )}
      </main>
    </>
  )
}
