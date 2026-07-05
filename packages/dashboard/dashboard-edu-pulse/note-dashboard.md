Sama-sama! Untuk halaman dashboard ini saya sarankan pendekatan berbeda dari halaman login: karena CSS-nya banyak pakai `:hover`, `::before`, `transition`, `max-height` animasi, dll — itu tidak praktis (dan tidak bisa) diubah jadi inline `style={{}}`. Jadi kita pakai **file CSS terpisah** + className, dan pecah jadi beberapa component kecil. Menu multilevel saya buat **data-driven** (struktur menu di file data, komponen menu men-generate otomatis) supaya benar-benar reusable dan terpisah.

## Struktur folder yang disarankan

```
app/
  dashboard/
    page.jsx
    dashboard.css
components/
  dashboard/
    TopNav.jsx
    Sidebar.jsx
    SidebarMenu.jsx      ← komponen menu multilevel
    sidebarMenuData.js   ← data struktur menu
    ProfileCard.jsx
    CurrentClassCard.jsx
    UpcomingClasses.jsx
    QuickLinks.jsx
    DeadlinesCard.jsx
    BottomNav.jsx
```

## 1. CSS — `app/dashboard/dashboard.css`

Copy semua isi `<style>` dari HTML kamu apa adanya ke file ini (class name tidak perlu diubah). Contoh awalannya:

```css
:root {
  --edu-blue: #1565C0;
  --edu-blue-light: #1976D2;
  --edu-accent: #2196F3;
  --edu-bg: #F4F6FA;
  --edu-card: #FFFFFF;
  --edu-text: #1A1A2E;
  --edu-muted: #6B7280;
  --edu-green: #22C55E;
  --edu-red: #EF4444;
  --edu-yellow: #F59E0B;
  --bottom-nav-h: 68px;
}

* { box-sizing: border-box; }

body {
  background: var(--edu-bg);
  font-family: 'Segoe UI', system-ui, sans-serif;
  color: var(--edu-text);
  margin: 0;
  padding: 0;
  min-height: 100vh;
}

/* ...lanjutkan seluruh CSS dari file HTML kamu (app-shell, top-nav, sidebar,
   sidebar-menu, submenu, submenu2, profile-card, current-card, class-item,
   quick-links-card, deadlines-card, bottom-nav, dst) — semua class name
   TETAP SAMA persis seperti aslinya, cukup paste ulang di sini. */
```

Saya tidak tulis ulang semua ratusan baris CSS-nya supaya jawaban tidak kepanjangan — tapi intinya **cukup copy-paste seluruh isi `<style>...</style>` dari HTML kamu apa adanya** ke `dashboard.css` ini, tidak ada yang perlu diubah.

## 2. Data menu — `components/dashboard/sidebarMenuData.js`

Ini yang bikin menu multilevel jadi "data-driven" — kalau nanti mau tambah/ubah menu, cukup edit file ini, tidak perlu sentuh komponen.

```js
const sidebarSections = [
  {
    label: "Main",
    items: [
      { type: "link", label: "Home", icon: "bi-house-door-fill", active: true, href: "#" },
      {
        type: "submenu",
        label: "Schedule",
        icon: "bi-calendar3",
        children: [
          { label: "Daily Schedule", href: "#" },
          { label: "Weekly View", href: "#" },
          { label: "Exam Calendar", href: "#" },
        ],
      },
      {
        type: "submenu",
        label: "Stats",
        icon: "bi-bar-chart-fill",
        children: [
          { label: "Performance Overview", href: "#" },
          {
            type: "submenu2",
            label: "Subject Stats",
            children: [
              { label: "Mathematics", href: "#" },
              { label: "Science", href: "#" },
              { label: "Literature", href: "#" },
              { label: "History", href: "#" },
            ],
          },
          { label: "Attendance Report", href: "#" },
          { label: "Ranking", href: "#" },
        ],
      },
      { type: "link", label: "Profile", icon: "bi-person-fill", href: "#" },
    ],
  },
  {
    label: "Academic",
    items: [
      {
        type: "submenu",
        label: "Grades",
        icon: "bi-award-fill",
        children: [
          { label: "Current Semester", href: "#" },
          {
            type: "submenu2",
            label: "Previous Semesters",
            children: [
              { label: "Semester 1 - 2024", href: "#" },
              { label: "Semester 2 - 2023", href: "#" },
              { label: "Semester 1 - 2023", href: "#" },
            ],
          },
          { label: "GPA Calculator", href: "#" },
          { label: "Transcripts", href: "#" },
        ],
      },
      {
        type: "submenu",
        label: "Homework",
        icon: "bi-journal-check",
        children: [
          { label: "Pending Tasks", href: "#", active: true },
          { label: "Submitted", href: "#" },
          { label: "Overdue", href: "#" },
        ],
      },
      { type: "link", label: "Events", icon: "bi-calendar-event-fill", href: "#" },
      { type: "link", label: "Fees", icon: "bi-credit-card-fill", href: "#" },
    ],
  },
  {
    label: "Support",
    items: [
      {
        type: "submenu",
        label: "Settings",
        icon: "bi-gear-fill",
        children: [
          { label: "Account", href: "#" },
          {
            type: "submenu2",
            label: "Notifications",
            children: [
              { label: "Push Notifications", href: "#" },
              { label: "Email Alerts", href: "#" },
              { label: "Reminder Settings", href: "#" },
            ],
          },
          { label: "Privacy", href: "#" },
          { label: "Language", href: "#" },
        ],
      },
      { type: "link", label: "Notifications", icon: "bi-bell-fill", href: "#" },
      { type: "link", label: "Logout", icon: "bi-box-arrow-right", href: "#" },
    ],
  },
];

export default sidebarSections;
```

## 3. Komponen menu multilevel — `components/dashboard/SidebarMenu.jsx`

Ini komponen terpisah yang kamu minta. Logic accordion level-1 (tutup yang lain saat buka satu) dan level-2 (independen, boleh lebih dari satu terbuka) mengikuti perilaku JS asli kamu.

```jsx
"use client";

import { useState } from "react";

export default function SidebarMenu({ sections }) {
  const [openMenu, setOpenMenu] = useState(null); // label level-1 yang sedang terbuka
  const [openSubmenu2, setOpenSubmenu2] = useState({}); // { label: true/false }

  function toggleMenu(label) {
    setOpenMenu((prev) => (prev === label ? null : label));
  }

  function toggleMenu2(label) {
    setOpenSubmenu2((prev) => ({ ...prev, [label]: !prev[label] }));
  }

  return (
    <nav className="sidebar-menu">
      {sections.map((section, sIdx) => (
        <div key={section.label}>
          <div className="section-label">{section.label}</div>

          {section.items.map((item, iIdx) => {
            if (item.type === "link") {
              return (
                <a key={iIdx} href={item.href || "#"} className={item.active ? "active" : ""}>
                  <i className={`bi ${item.icon}`}></i> {item.label}
                </a>
              );
            }

            if (item.type === "submenu") {
              const isOpen = openMenu === item.label;
              return (
                <div key={iIdx}>
                  <div
                    className={`has-submenu ${isOpen ? "open" : ""}`}
                    onClick={() => toggleMenu(item.label)}
                  >
                    <i className={`bi ${item.icon} menu-icon`}></i>
                    <span className="menu-label">{item.label}</span>
                    <i className="bi bi-chevron-right arrow"></i>
                  </div>

                  <div className={`submenu ${isOpen ? "open" : ""}`}>
                    {item.children.map((child, cIdx) => {
                      if (child.type === "submenu2") {
                        const isOpen2 = !!openSubmenu2[child.label];
                        return (
                          <div key={cIdx}>
                            <div
                              className={`has-submenu2 ${isOpen2 ? "open" : ""}`}
                              onClick={() => toggleMenu2(child.label)}
                            >
                              <span className="sub-label">{child.label}</span>
                              <i className="bi bi-chevron-right arrow2"></i>
                            </div>
                            <div className={`submenu2 ${isOpen2 ? "open" : ""}`}>
                              {child.children.map((sub, subIdx) => (
                                <a key={subIdx} href={sub.href || "#"}>
                                  {sub.label}
                                </a>
                              ))}
                            </div>
                          </div>
                        );
                      }

                      return (
                        
                          key={cIdx}
                          href={child.href || "#"}
                          className={child.active ? "active" : ""}
                        >
                          {child.label}
                        </a>
                      );
                    })}
                  </div>
                </div>
              );
            }

            return null;
          })}

          {sIdx < sections.length - 1 && <hr className="divider" />}
        </div>
      ))}
    </nav>
  );
}
```

## 4. `components/dashboard/Sidebar.jsx`

```jsx
"use client";

import SidebarMenu from "./SidebarMenu";
import sidebarSections from "./sidebarMenuData";

export default function Sidebar({ open, onClose }) {
  return (
    <>
      <div className={`sidebar-overlay ${open ? "open" : ""}`} onClick={onClose}></div>

      <div className={`sidebar ${open ? "open" : ""}`}>
        <button className="sidebar-close" onClick={onClose}>
          <i className="bi bi-x-lg"></i>
        </button>

        <div className="sidebar-header">
          <img src="https://i.pravatar.cc/100?img=11" className="s-avatar" alt="Alex" />
          <h6>Alex Johnson</h6>
          <small>Grade 11 • Section B • #882910</small>
        </div>

        <SidebarMenu sections={sidebarSections} />
      </div>
    </>
  );
}
```

## 5. `components/dashboard/TopNav.jsx`

```jsx
export default function TopNav({ onOpenSidebar }) {
  return (
    <div className="top-nav">
      <div className="d-flex align-items-center gap-2">
        <button className="hamburger-btn" onClick={onOpenSidebar}>
          <i className="bi bi-list"></i>
        </button>
        <div className="brand">
          <div className="logo-box">
            <i className="bi bi-mortarboard-fill"></i>
          </div>
          EduPulse
        </div>
      </div>
      <div className="nav-right">
        <button className="notif-btn">
          <i className="bi bi-bell"></i>
          <span className="dot"></span>
        </button>
        <img src="https://i.pravatar.cc/80?img=11" className="avatar" alt="Alex" />
      </div>
    </div>
  );
}
```

## 6. `components/dashboard/ProfileCard.jsx`

```jsx
export default function ProfileCard() {
  return (
    <div className="profile-card">
      <div className="p-avatar-wrap">
        <img src="https://i.pravatar.cc/150?img=11" className="p-avatar" alt="Alex Johnson" />
        <span className="online-dot"></span>
      </div>
      <h5>Alex Johnson</h5>
      <p className="sub">Grade 11 - Section B &bull; Student ID: #882910</p>
      <div>
        <span className="badge-pill">HONOR ROLL</span>
        <span className="badge-pill">FOOTBALL TEAM</span>
      </div>
      <div className="stat-row">
        <div className="stat-item text-center">
          <div className="val">3.8</div>
          <div className="lbl">GPA</div>
        </div>
        <div className="stat-item text-center">
          <div className="val">96%</div>
          <div className="lbl">Attendance</div>
        </div>
      </div>
    </div>
  );
}
```

## 7. `components/dashboard/CurrentClassCard.jsx`

```jsx
export default function CurrentClassCard() {
  return (
    <div className="current-card">
      <div className="d-flex justify-content-between align-items-center mb-1">
        <span className="period-label">Current Period</span>
        <span className="live-badge">LIVE NOW</span>
      </div>
      <h4>Advanced Mathematics</h4>
      <p className="room-info">
        <i className="bi bi-geo-alt-fill me-1"></i>Room 302 &bull; 09:00 AM – 10:30 AM
      </p>
      <div className="d-flex align-items-center">
        <a href="#" className="btn-join">
          <i className="bi bi-camera-video-fill"></i> Join Virtual Class
        </a>
        <button className="btn-doc">
          <i className="bi bi-file-earmark-text"></i>
        </button>
      </div>
    </div>
  );
}
```

## 8. `components/dashboard/UpcomingClasses.jsx`

```jsx
const classes = [
  {
    title: "Organic Chemistry",
    time: "10:45 AM – 12:15 PM • Lab 12",
    icon: "☀️",
    iconClass: "chem",
  },
  {
    title: "World Literature",
    time: "01:00 PM – 02:30 PM • Room 105",
    icon: <i className="bi bi-book-fill" style={{ color: "#0EA5E9" }}></i>,
    iconClass: "lit",
  },
];

export default function UpcomingClasses() {
  return (
    <>
      <div className="section-title">
        <h6>Upcoming Classes</h6>
        <a href="#">Full Schedule</a>
      </div>

      {classes.map((c, i) => (
        <div className="class-item" key={i}>
          <div className={`class-icon ${c.iconClass}`}>{c.icon}</div>
          <div>
            <p className="ci-title">{c.title}</p>
            <p className="ci-sub">{c.time}</p>
          </div>
        </div>
      ))}
    </>
  );
}
```

## 9. `components/dashboard/QuickLinks.jsx`

```jsx
const links = [
  { icon: "bi-star", label: "Grades" },
  { icon: "bi-journal-text", label: "Homework" },
  { icon: "bi-calendar3", label: "Events" },
  { icon: "bi-wallet2", label: "Fees" },
];

export default function QuickLinks() {
  return (
    <>
      <div className="section-title" style={{ paddingTop: 14 }}>
        <h6>&nbsp;</h6>
      </div>
      <div className="quick-links-card">
        <div className="ql-label">Quick Links</div>
        <div className="ql-grid">
          {links.map((l) => (
            <a href="#" className="ql-item" key={l.label}>
              <i className={`bi ${l.icon}`}></i>
              <span>{l.label}</span>
            </a>
          ))}
        </div>
      </div>
    </>
  );
}
```

## 10. `components/dashboard/DeadlinesCard.jsx`

```jsx
const deadlines = [
  { title: "History Essay", due: "Due in 2 hours", level: "urgent" },
  { title: "Physics Lab Report", due: "Due tomorrow", level: "soon" },
  { title: "Math Quiz Prep", due: "Due Friday", level: "later" },
];

export default function DeadlinesCard() {
  return (
    <div className="deadlines-card">
      <h6>Deadlines</h6>

      {deadlines.map((d) => (
        <div className="dl-item" key={d.title}>
          <div className={`dl-bar ${d.level}`}></div>
          <div>
            <p className="dl-title">{d.title}</p>
            <p className={`dl-due ${d.level}`}>{d.due}</p>
          </div>
        </div>
      ))}

      <a href="#" className="btn-view-all">
        View All Tasks
      </a>
    </div>
  );
}
```

## 11. `components/dashboard/BottomNav.jsx`

```jsx
export default function BottomNav() {
  return (
    <div className="bottom-nav">
      <a href="#" className="bn-item active">
        <i className="bi bi-house-door-fill"></i>
        <span>Home</span>
      </a>
      <a href="#" className="bn-item">
        <i className="bi bi-calendar3"></i>
        <span>Schedule</span>
      </a>
      <div className="bn-fab">
        <i className="bi bi-plus-lg"></i>
      </div>
      <a href="#" className="bn-item">
        <i className="bi bi-bar-chart-fill"></i>
        <span>Stats</span>
      </a>
      <a href="#" className="bn-item">
        <i className="bi bi-person-fill"></i>
        <span>Profile</span>
      </a>
    </div>
  );
}
```

## 12. Halaman utama — `app/dashboard/page.jsx`

```jsx
"use client";

import { useState } from "react";
import "./dashboard.css";

import TopNav from "@/components/dashboard/TopNav";
import Sidebar from "@/components/dashboard/Sidebar";
import ProfileCard from "@/components/dashboard/ProfileCard";
import CurrentClassCard from "@/components/dashboard/CurrentClassCard";
import UpcomingClasses from "@/components/dashboard/UpcomingClasses";
import QuickLinks from "@/components/dashboard/QuickLinks";
import DeadlinesCard from "@/components/dashboard/DeadlinesCard";
import BottomNav from "@/components/dashboard/BottomNav";

export default function DashboardPage() {
  const [sidebarOpen, setSidebarOpen] = useState(false);

  return (
    <div className="app-shell">
      <Sidebar open={sidebarOpen} onClose={() => setSidebarOpen(false)} />
      <TopNav onOpenSidebar={() => setSidebarOpen(true)} />

      <div className="page-content">
        <ProfileCard />
        <CurrentClassCard />
        <UpcomingClasses />
        <QuickLinks />
        <DeadlinesCard />
      </div>

      <BottomNav />
    </div>
  );
}
```

## Catatan penting

1. **Alias `@/`** — import di atas pakai `@/components/...`. Pastikan ada `jsconfig.json` di root project seperti ini (kalau belum ada):
```json
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": { "@/*": ["./*"] }
  }
}
```
Kalau tidak mau setup ini, ganti semua import jadi path relatif, contoh `../../components/dashboard/TopNav`.

2. **Import CSS di page, bukan global** — `import "./dashboard.css"` di `page.jsx` sudah valid di App Router (global CSS boleh diimport di component/page manapun dalam folder `app`), jadi style-nya hanya aktif dan ter-bundle untuk halaman dashboard ini.

3. **Update redirect login** — di component login sebelumnya saya arahkan ke `/edupulse`, sesuaikan jadi:
```js
router.push("/dashboard");
```

4. Semua `onclick` di HTML asli sudah diganti `onClick` + `useState`, tidak ada lagi manipulasi `classList` manual — React yang mengatur render ulang berdasarkan state.

Mau saya lanjutkan juga membuatkan proteksi route-nya (supaya `/dashboard` tidak bisa diakses kalau belum login)?