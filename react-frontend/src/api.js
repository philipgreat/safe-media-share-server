const KEY_STORAGE = 'mg_admin_key'

export function getAdminKey() {
  return sessionStorage.getItem(KEY_STORAGE) || ''
}

export function setAdminKey(v) {
  if (v) sessionStorage.setItem(KEY_STORAGE, v)
  else sessionStorage.removeItem(KEY_STORAGE)
}

export function normalizeToken(raw) {
  let s = String(raw || '').trim()
  if (!s) return ''
  try {
    if (s.includes('://')) {
      const u = new URL(s)
      s = u.pathname
    }
  } catch (_) {}
  s = s.replace(/^\/+/, '')
  const m = s.match(/(?:invite\/)?(INV-[A-Za-z0-9_-]+)/i)
  return m ? m[1] : s
}

export async function api(path, opts = {}) {
  const res = await fetch(path, {
    ...opts,
    headers: {
      'X-Admin-Key': getAdminKey(),
      ...(opts.headers || {}),
    },
  })
  const body = await res.json().catch(() => ({}))
  return { res, body }
}

export function joinExternal(base, path) {
  const b = String(base || '').replace(/\/+$/, '')
  const p = path.startsWith('/') ? path : `/${path}`
  return b ? `${b}${p}` : p
}

export function formatSize(n) {
  if (n == null) return ''
  if (n < 1024) return `${n} B`
  if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`
  return `${(n / 1024 / 1024).toFixed(2)} MB`
}

export function kindOf(contentType, filePath) {
  const t = (contentType || '').toLowerCase()
  const p = (filePath || '').toLowerCase()
  if (t.startsWith('video/')) return 'video'
  if (t.startsWith('audio/')) return 'audio'
  if (t.startsWith('image/')) return 'image'
  if (t.includes('pdf') || t.startsWith('text/') || p.endsWith('.txt') || p.endsWith('.md') || p.endsWith('.pdf')) return 'doc'
  if (p.endsWith('.zip') || p.endsWith('.tar') || p.endsWith('.gz') || p.endsWith('.7z') || t.includes('zip')) return 'archive'
  return 'other'
}

export function kindLabel(kind) {
  return (
    {
      video: '视频',
      audio: '音频',
      image: '图片',
      doc: '文档',
      archive: '压缩包',
      other: '文件',
    }[kind] || '文件'
  )
}

export async function copyText(text) {
  if (navigator.clipboard?.writeText) {
    await navigator.clipboard.writeText(text)
    return true
  }
  const ta = document.createElement('textarea')
  ta.value = text
  document.body.appendChild(ta)
  ta.select()
  document.execCommand('copy')
  ta.remove()
  return true
}
