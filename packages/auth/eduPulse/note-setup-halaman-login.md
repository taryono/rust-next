Oke, saya bantu convert ke component Next.js 16 (App Router + JSX, tanpa TypeScript). Berikut strukturnya:

## 1. Setup Bootstrap & Icons

Karena tidak pakai TypeScript, cukup pasang CSS-nya. Termudah pakai CDN lewat `app/layout.jsx` (tidak perlu install npm
package):

**`app/layout.jsx`**
```jsx
export const metadata = {
title: "EduPulse",
description: "Student login portal",
};

export default function RootLayout({ children }) {
return (
<html lang="en">

<head>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css" rel="stylesheet" />
    <link href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.min.css" rel="stylesheet" />
</head>

<body>{children}</body>

</html>
);
}
```

Kalau nanti butuh Bootstrap JS (dropdown, modal, dll), baru tambahkan lewat `next/script` — untuk halaman login ini
sebenarnya tidak perlu karena semua interaksi (toggle password, validasi) kita handle dengan React state.

## 2. Component Login

Karena ada `useState` dan event handler, wajib `"use client"` di baris paling atas.

**`app/page.jsx`** (biar langsung tampil saat load ke `/`)
```jsx
"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";

export default function LoginPage() {
const router = useRouter();

const [email, setEmail] = useState("");
const [password, setPassword] = useState("");
const [showPassword, setShowPassword] = useState(false);
const [errors, setErrors] = useState({ email: false, password: false });
const [loginError, setLoginError] = useState(false);

function togglePassword() {
setShowPassword((prev) => !prev);
}

function handleLogin(e) {
e.preventDefault();

const trimmedEmail = email.trim();
const trimmedPw = password.trim();

const newErrors = {
email: !trimmedEmail,
password: !trimmedPw,
};
setErrors(newErrors);
setLoginError(false);

if (newErrors.email || newErrors.password) return;

// Simulasi cek login — ganti dengan pemanggilan API asli
if (trimmedEmail === "alex@edupulse.com" && trimmedPw === "password123") {
router.push("/edupulse");
} else {
setLoginError(true);
}
}

return (
<div style={{ background: "#F4F6FA" , minHeight: "100vh" , display: "flex" , alignItems: "center" ,
    justifyContent: "center" , padding: "24px 16px" , fontFamily: "'Segoe UI', system-ui, sans-serif" , }}>
    <div className="auth-shell" style={{ width: "100%" , maxWidth: 420 }}>
        {/* Logo */}
        <div className="auth-logo" style={{ display: "flex" , alignItems: "center" , justifyContent: "center" , gap: 10,
            marginBottom: 28, }}>
            <div style={{ width: 44, height: 44, background: "#1565C0" , borderRadius: 12, display: "flex" ,
                alignItems: "center" , justifyContent: "center" , color: "#fff" , fontSize: "1.3rem" ,
                boxShadow: "0 4px 14px rgba(21,101,192,.35)" , }}>
                <i className="bi bi-mortarboard-fill" />
            </div>
            <span style={{ fontSize: "1.5rem" , fontWeight: 800, color: "#1565C0" , letterSpacing: "-.5px" , }}>
                EduPulse
            </span>
        </div>

        {/* Card */}
        <div className="auth-card" style={{ background: "#fff" , borderRadius: 20, padding: "32px 28px 28px" ,
            boxShadow: "0 4px 24px rgba(0,0,0,.08)" , }}>
            <h4 style={{ fontSize: "1.2rem" , fontWeight: 800, color: "#1A1A2E" , marginBottom: 4 }}>
                Welcome back! 👋
            </h4>
            <p style={{ fontSize: ".82rem" , color: "#6B7280" , marginBottom: 24 }}>
                Login to your student account to continue
            </p>

            {loginError && (
            <div className="alert alert-danger d-flex align-items-center gap-2 py-2 px-3 mb-3" style={{
                fontSize: ".83rem" , borderRadius: 10 }}>
                <i className="bi bi-exclamation-circle-fill" />
                <span>Invalid email or password.</span>
            </div>
            )}

            <form onSubmit={handleLogin}>
                {/* Email */}
                <label className="form-label" style={{ fontSize: ".8rem" , fontWeight: 600, color: "#374151" }}>
                    Email / Student ID
                </label>
                <div style={{ position: "relative" , marginBottom: 16 }}>
                    <i className="bi bi-person" style={{ position: "absolute" , left: 14, top: "50%" ,
                        transform: "translateY(-50%)" , color: "#9CA3AF" , }} />
                    <input type="text" className={`form-control ${errors.email ? "is-invalid" : "" }`}
                        placeholder="Enter your email or ID" value={email} onChange={(e)=> setEmail(e.target.value)}
                    style={{ paddingLeft: 40, borderRadius: 10, height: 46, fontSize: ".9rem" }}
                    />
                </div>
                {errors.email && (
                <div style={{ fontSize: ".76rem" , color: "#EF4444" , marginTop: -12, marginBottom: 12 }}>
                    Please enter your email or Student ID.
                </div>
                )}

                {/* Password */}
                <label className="form-label" style={{ fontSize: ".8rem" , fontWeight: 600, color: "#374151" }}>
                    Password
                </label>
                <div style={{ position: "relative" , marginBottom: 16 }}>
                    <i className="bi bi-lock" style={{ position: "absolute" , left: 14, top: "50%" ,
                        transform: "translateY(-50%)" , color: "#9CA3AF" , }} />
                    <input type={showPassword ? "text" : "password" } className={`form-control ${errors.password
                        ? "is-invalid" : "" }`} placeholder="Enter your password" value={password} onChange={(e)=>
                    setPassword(e.target.value)}
                    style={{ paddingLeft: 40, borderRadius: 10, height: 46, fontSize: ".9rem" }}
                    />
                    <button type="button" onClick={togglePassword} style={{ position: "absolute" , right: 13, top: "50%"
                        , transform: "translateY(-50%)" , background: "none" , border: "none" , color: "#9CA3AF" ,
                        cursor: "pointer" , }}>
                        <i className={`bi ${showPassword ? "bi-eye-slash" : "bi-eye" }`} />
                    </button>
                </div>
                {errors.password && (
                <div style={{ fontSize: ".76rem" , color: "#EF4444" , marginTop: -12, marginBottom: 12 }}>
                    Please enter your password.
                </div>
                )}

                {/* Remember / Forgot */}
                <div style={{ display: "flex" , justifyContent: "space-between" , alignItems: "center" , marginTop: 8,
                    marginBottom: 20, }}>
                    <div className="form-check">
                        <input className="form-check-input" type="checkbox" id="rememberMe" />
                        <label className="form-check-label" htmlFor="rememberMe" style={{ fontSize: ".82rem" ,
                            color: "#374151" }}>
                            Remember me
                        </label>
                    </div>
                    <a href="#" style={{ fontSize: ".82rem" , color: "#2196F3" , fontWeight: 600, textDecoration: "none"
                        }}>
                        Forgot password?
                    </a>
                </div>

                {/* Submit */}
                <button type="submit" style={{ width: "100%" , background: "#1565C0" , color: "#fff" , border: "none" ,
                    borderRadius: 12, height: 48, fontSize: ".95rem" , fontWeight: 700, cursor: "pointer" ,
                    display: "flex" , alignItems: "center" , justifyContent: "center" , gap: 8, }}>
                    <i className="bi bi-box-arrow-in-right" /> Login
                </button>
            </form>

            <div style={{ display: "flex" , alignItems: "center" , gap: 12, margin: "20px 0" , fontSize: ".78rem" ,
                color: "#6B7280" }}>
                <div style={{ flex: 1, height: 1, background: "#E5E7EB" }} />
                or continue with
                <div style={{ flex: 1, height: 1, background: "#E5E7EB" }} />
            </div>

            <button style={{ width: "100%" , background: "#F9FAFB" , border: "1.5px solid #E5E7EB" , borderRadius: 12,
                height: 46, fontSize: ".88rem" , fontWeight: 600, color: "#374151" , cursor: "pointer" , display: "flex"
                , alignItems: "center" , justifyContent: "center" , gap: 10, marginBottom: 10, }}>
                <img src="https://www.gstatic.com/firebasejs/ui/2.0.0/images/auth/google.svg" alt="Google" width={20}
                    height={20} />
                Continue with Google
            </button>
            <button style={{ width: "100%" , background: "#F9FAFB" , border: "1.5px solid #E5E7EB" , borderRadius: 12,
                height: 46, fontSize: ".88rem" , fontWeight: 600, color: "#374151" , cursor: "pointer" , display: "flex"
                , alignItems: "center" , justifyContent: "center" , gap: 10, }}>
                <i className="bi bi-microsoft" style={{ color: "#00A4EF" , fontSize: "1.1rem" }} />
                Continue with Microsoft
            </button>
        </div>

        <div style={{ textAlign: "center" , marginTop: 20, fontSize: ".83rem" , color: "#6B7280" }}>
            Don&apos;t have an account?{" "}
            <a href="/register" style={{ color: "#1565C0" , fontWeight: 700, textDecoration: "none" }}>
                Sign up here
            </a>
        </div>
    </div>
</div>
);
}
```

## 3. Kenapa ini akan langsung tampil?

Di App Router, `app/page.jsx` otomatis jadi route `/`. Jadi selama file di atas kamu taruh persis di `app/page.jsx`
(bukan di dalam folder lain), Next.js akan menampilkannya begitu server dijalankan dan user membuka
`http://localhost:3000`.

Beberapa catatan penting perubahan dari HTML asli:
- **`onclick` → `onClick`**, dan logic-nya dipindah ke `useState` + fungsi biasa (bukan manipulasi DOM langsung, karena
React yang mengatur render).
- **`window.location.href`** diganti `router.push()` dari `next/navigation` — ini cara redirect yang benar di App
Router.
- Saya taruh style langsung sebagai inline `style={{}}` supaya cepat jalan tanpa file CSS terpisah. Kalau kamu mau lebih
rapi, styling itu sebaiknya dipindah ke file `app/page.module.css` dan pakai className — bisa saya bantu pisahkan kalau
mau.
- Elemen form dibungkus `<form onSubmit={...}>` (bukan tombol lepas dengan `onClick`), supaya Enter di keyboard juga
    bisa submit — lebih sesuai standar web.

    Mau saya pisahkan CSS-nya ke CSS Modules biar tidak numpuk di inline style?