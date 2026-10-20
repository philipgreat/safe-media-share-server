import { useEffect, useState } from 'react'
import { NavLink, Navigate, Route, Routes, useNavigate } from 'react-router-dom'
import { api, getAdminKey, setAdminKey } from '../api.js'
import AdminMedia from './AdminMedia.jsx'
import AdminWhitelist from './AdminWhitelist.jsx'
import AdminSpent from './AdminSpent.jsx'
import AdminSettings from './AdminSettings.jsx'

function Login({ onOk }) {
  const [key, setKey] = useState('')
  const [msg, setMsg] = useState(null)
  const [busy, setBusy] = useState(false)

  async function submit(ev) {
    ev.preventDefault()
    const v = key.trim()
    if (!v) {
      setMsg({ kind: 'err', text: 'Enter admin key' })
      return
    }
    setBusy(true)
    setMsg({ kind: 'warn', text: 'Verifying…' })
    setAdminKey(v)
    try {
      const { res, body } = await api('/admin/api/ping')
      if (res.ok) {
        onOk()
      } else {
        setAdminKey('')
        setMsg({ kind: 'err', text: body.message || `Invalid key (HTTP ${res.status})` })
      }
    } catch (e) {
      setAdminKey('')
      setMsg({ kind: 'err', text: `Request failed: ${e}` })
    } finally {
      setBusy(false)
    }
  }

  return (
    <div className="login-wrap">
      <section className="login-card">
        <h1>Admin access</h1>
        <p className="muted small" style={{ margin: 0 }}>
          Super key from <code>MEDIA_GUARD_ADMIN_KEY</code>
        </p>
        <form onSubmit={submit}>
          <div>
            <div className="field-label">Admin key</div>
            <input
              type="password"
              value={key}
              onChange={(e) => setKey(e.target.value)}
              required
            />
          </div>
          <button className="btn" type="submit" disabled={busy}>
            {busy ? 'Signing in…' : 'Sign in'}
          </button>
        </form>
        {msg && <div className={`notice ${msg.kind}`}>{msg.text}</div>}
      </section>
    </div>
  )
}

export default function AdminApp() {
  const [authed, setAuthed] = useState(null)
  const [settings, setSettings] = useState({ externalBaseUrl: '' })
  const navigate = useNavigate()

  async function check() {
    if (!getAdminKey()) {
      setAuthed(false)
      return
    }
    try {
      const { res } = await api('/admin/api/ping')
      if (res.ok) {
        setAuthed(true)
        const s = await api('/admin/api/settings')
        const rows = s.body?.data || []
        const found =
          rows.find((r) => r.settingKey === 'external_base_url') ||
          rows[0] ||
          {}
        setSettings({ externalBaseUrl: found.externalBaseUrl || found.settingValue || '' })
      } else {
        setAdminKey('')
        setAuthed(false)
      }
    } catch {
      setAuthed(false)
    }
  }

  useEffect(() => {
    check()
  }, [])

  function logout() {
    setAdminKey('')
    setAuthed(false)
    navigate('/admin')
  }

  if (authed === null) {
    return (
      <div className="login-wrap">
        <p className="muted">Checking session…</p>
      </div>
    )
  }

  if (!authed) {
    return <Login onOk={() => check()} />
  }

  return (
    <div className="admin-app">
      <aside className="sidebar">
        <div className="brand">Media Guard</div>
        <nav>
          <NavLink to="/admin" end className={({ isActive }) => `nav-item${isActive ? ' active' : ''}`}>
            Media
          </NavLink>
          <NavLink
            to="/admin/whitelist"
            className={({ isActive }) => `nav-item${isActive ? ' active' : ''}`}
          >
            Whitelist
          </NavLink>
          <NavLink
            to="/admin/spent"
            className={({ isActive }) => `nav-item${isActive ? ' active' : ''}`}
          >
            Spent invites
          </NavLink>
          <NavLink
            to="/admin/settings"
            className={({ isActive }) => `nav-item${isActive ? ' active' : ''}`}
          >
            Settings
          </NavLink>
        </nav>
        <div className="sidebar-foot">
          <a href="/">← Public site</a>
          <button className="btn secondary" type="button" onClick={logout}>
            Sign out
          </button>
        </div>
      </aside>
      <main className="admin-main">
        <Routes>
          <Route index element={<AdminMedia settings={settings} />} />
          <Route path="whitelist" element={<AdminWhitelist />} />
          <Route path="spent" element={<AdminSpent />} />
          <Route
            path="settings"
            element={<AdminSettings settings={settings} onSaved={(next) => setSettings(next)} />}
          />
          <Route path="*" element={<Navigate to="/admin" replace />} />
        </Routes>
      </main>
    </div>
  )
}
