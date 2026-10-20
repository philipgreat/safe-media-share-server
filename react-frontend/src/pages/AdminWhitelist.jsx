import { useEffect, useState } from 'react'
import { api } from '../api.js'

export default function AdminWhitelist() {
  const [rows, setRows] = useState([])
  const [error, setError] = useState('')

  useEffect(() => {
    ;(async () => {
      try {
        const { res, body } = await api('/admin/api/whitelist')
        if (!res.ok) {
          setError(body.message || 'Load failed')
          return
        }
        setRows(body.data || [])
      } catch (e) {
        setError(String(e))
      }
    })()
  }, [])

  return (
    <>
      <header className="page-head">
        <h2>Whitelist</h2>
        <p className="muted">Active, non-expired IP grants</p>
      </header>
      {error && <div className="notice err">{error}</div>}
      <div className="table-wrap">
        <table className="admin-table">
          <thead>
            <tr>
              <th>IP</th>
              <th>Media</th>
              <th>Expires</th>
              <th>Note</th>
              <th>Grant</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((it) => (
              <tr key={it.grantId}>
                <td className="tag solid">{it.clientIp}</td>
                <td>
                  <div>{it.mediaName || `#${it.mediaAssetId}`}</div>
                  <div className="code faint">{it.mediaFile}</div>
                </td>
                <td className="code">{it.expireAt}</td>
                <td className="muted">{it.grantNote}</td>
                <td className="tag">#{it.grantId}</td>
              </tr>
            ))}
          </tbody>
        </table>
        {!error && rows.length === 0 && <p className="empty">Whitelist is empty.</p>}
      </div>
    </>
  )
}
