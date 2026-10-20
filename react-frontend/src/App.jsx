import { Routes, Route, Navigate } from 'react-router-dom'
import UserHome from './pages/UserHome.jsx'
import ViewPage from './pages/ViewPage.jsx'
import AdminApp from './pages/AdminApp.jsx'

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<UserHome />} />
      <Route path="/view/:ref" element={<ViewPage />} />
      <Route path="/admin/*" element={<AdminApp />} />
      <Route path="*" element={<Navigate to="/" replace />} />
    </Routes>
  )
}
