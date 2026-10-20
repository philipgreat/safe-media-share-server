import { useEffect, useState } from 'react'
import { api, joinExternal } from '../api.js'

export default function AdminSettings({ settings, onSaved }) {
  const [value, setValue] = useState(settings?.externalBaseUrl || '')
  const [msg, setMsg] = useState(null)
  const [busy, setBusy] = useState(false)

  useEffect(() => {
    setValue(settings?.externalBaseUrl || '')
  }, [settings])

  async function submit(ev) {
    ev.preventDefault()
    setBusy(true)
    setMsg({ kind: 'warn', text: 'Saving…' })
    try {
      const { res, body } = await api('/admin/api/settings', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ externalBaseUrl: value.trim() }),
      })
      if (res.ok) {
        const saved = body?.data?.[0]?.externalBaseUrl || value.trim()
        onSaved?.({ externalBaseUrl: saved })
        setValue(saved)
        setMsg({ kind: 'ok', text: 'Saved to database. Links will use this prefix.' })
      } else {
        setMsg({ kind: 'err', text: body.message || 'Save failed' })
      }
    } catch (e) {
      setMsg({ kind: 'err', text: String(e) })
    } finally {
      setBusy(false)
    }
  }

  const previewBase = value.trim()
    ? value.trim().includes('://')
      ? value.trim().replace(/\/+$/, '')
      : `https://${value.trim().replace(/\/+$/, '')}`
    : typeof location !== 'undefined'
      ? location.origin
      : ''

  return (
    <>
      <header className="page-head">
        <h2>Settings</h2>
        <p className="muted">Runtime configuration stored in the database</p>
      </header>
      <section className="settings-card">
        <h3>External base URL</h3>
        <p className="muted small" style={{ margin: 0 }}>
          Public prefix for share/open links. Internal address may differ.
          Example: <code>https://media.example.com</code>
        </p>
        <form className="settings-form" onSubmit={submit}>
          <div style={{ flex: 1, minWidth: 0 }}>
            <div className="field-label">Prefix</div>
            <input
              value={value}
              onChange={(e) => setValue(e.target.value)}
              placeholder="https://media.example.com"
              spellCheck={false}
            />
          </div>
          <div style={{ alignSelf: 'end' }}>
            <button className="btn" type="submit" disabled={busy}>
              {busy ? 'Saving…' : 'Save'}
            </button>
          </div>
        </form>
        {msg && <div className={`notice ${msg.kind}`}>{msg.text}</div>}
        <div className="preview">
          <div className="field-label" style={{ marginBottom: 4 }}>
            Preview
          </div>
          {joinExternal(previewBase, '/invite/INV-…')}
        </div>
      </section>
    </>
  )
}
