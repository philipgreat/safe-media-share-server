import { useEffect, useState } from 'react'
import { api } from '../api.js'

export default function AdminSpent() {
  const [rows, setRows] = useState([])
  const [error, setError] = useState('')

  useEffect(() => {
    ;(async () => {
      try {
        const { res, body } = await api('/admin/api/spent-invites')
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
        <h2>Spent invites</h2>
        <p className="muted">Disposed by second click — code is dead</p>
      </header>
      {error && <div className="notice err">{error}</div>}
      <div className="table-wrap">
        <table className="admin-table">
          <thead>
            <tr>
              <th>Token</th>
              <th>Media</th>
              <th>First click</th>
              <th>Second click</th>
              <th>State</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((it) => (
              <tr key={it.inviteId}>
                <td>
                  <div className="code">{it.inviteToken}</div>
                  <div className="faint small">id {it.inviteId}</div>
                </td>
                <td>
                  <div>{it.mediaName || `#${it.mediaAssetId}`}</div>
                  <div className="code faint">{it.mediaFile}</div>
                </td>
                <td>
                  <div className="tag solid">{it.firstClientIp}</div>
                  <div className="faint small">{it.firstClickAt}</div>
                </td>
                <td>
                  <div className="tag solid">{it.secondClientIp}</div>
                  <div className="faint small">{it.secondClickAt}</div>
                </td>
                <td>
                  <span className="tag solid">SPENT</span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        {!error && rows.length === 0 && <p className="empty">No spent invites yet.</p>}
      </div>
    </>
  )
}
