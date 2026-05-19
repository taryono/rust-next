"use client";

import { useState } from "react";

// ─── Data ────────────────────────────────────────────────────────────────────

const SEKOLAH_UNITS = [
  { v: "paud", l: "PAUD", d: "Pendidikan Anak Usia Dini", icon: "🧸" },
  { v: "tk", l: "Taman Kanak-Kanak (TK)", d: "Kelompok Bermain & TK", icon: "🎨" },
  { v: "sd", l: "Sekolah Dasar (SD)", d: "Kelas 1 – 6", icon: "📚" },
  { v: "smp", l: "SMP / MTs", d: "Kelas 7 – 9", icon: "🏫" },
  { v: "sma", l: "SMA / SMK / MA", d: "Kelas 10 – 12", icon: "🎓" },
];

const KAMPUS_UNITS = [
  { v: "fk", l: "Fakultas Kedokteran", d: "Kedokteran Umum, Keperawatan, Farmasi", icon: "🏥" },
  { v: "fe", l: "Fakultas Ekonomi", d: "Manajemen, Akuntansi, Ekonomi", icon: "📊" },
  { v: "ft", l: "Fakultas Teknik", d: "Sipil, Mesin, Elektro, Informatika", icon: "⚙️" },
  { v: "fh", l: "Fakultas Hukum", d: "Ilmu Hukum & Pidana", icon: "⚖️" },
  { v: "fis", l: "Fakultas Ilmu Sosial", d: "Komunikasi, Sosiologi, Psikologi", icon: "👥" },
  { v: "fp", l: "Fakultas Pertanian", d: "Agronomi, Peternakan, Kehutanan", icon: "🌱" },
];

const ALL_FEATURES = [
  { v: "absensi", l: "Absensi Digital", icon: "📅" },
  { v: "raport", l: "Rapor & Nilai", icon: "📝" },
  { v: "spp", l: "Pembayaran SPP", icon: "💳" },
  { v: "perpus", l: "Perpustakaan", icon: "📖" },
  { v: "ekskul", l: "Ekstra Kurikuler", icon: "🏆" },
  { v: "bk", l: "Bimbingan Konseling", icon: "🤝" },
  { v: "inventaris", l: "Inventaris Aset", icon: "📦" },
  { v: "alumni", l: "Data Alumni", icon: "🪪" },
  { v: "elearning", l: "E-Learning / LMS", icon: "💻" },
  { v: "beasiswa", l: "Beasiswa & Bantuan", icon: "🎖️" },
  { v: "keuangan", l: "Laporan Keuangan", icon: "📈" },
  { v: "komunikasi", l: "Komunikasi Orang Tua", icon: "💬" },
];

const PROVINCES = [
  "DKI Jakarta", "Jawa Barat", "Jawa Tengah", "Jawa Timur",
  "Banten", "DI Yogyakarta", "Bali", "Sumatera Utara",
  "Sulawesi Selatan", "Lainnya",
];

const PORTAL_OPTIONS = [
  { v: "pmb", icon: "📋", title: "Portal Pendaftaran (PMB)", desc: "Formulir online, seleksi, pengumuman" },
  { v: "student", icon: "🪪", titleSekolah: "Portal Siswa", titleKampus: "Portal Mahasiswa", desc: "Nilai, jadwal, kehadiran, tugas" },
  { v: "admin", icon: "🛡️", titleSekolah: "Portal Guru & Admin", titleKampus: "Portal Dosen & Admin", desc: "Manajemen data, laporan, keuangan" },
  { v: "mobile", icon: "📱", title: "Mobile Apps", desc: "Android & iOS untuk siswa & orang tua" },
];

const STEP_LABELS = ["Sistem", "Unit", "Profil", "Portal", "Fitur", "Mulai"];

// ─── Helper ───────────────────────────────────────────────────────────────────

const toggle = (arr, v) =>
  arr.includes(v) ? arr.filter((x) => x !== v) : [...arr, v];

// ─── Sub-components ───────────────────────────────────────────────────────────

function StepDot({ index, current }) {
  const isDone = index < current;
  const isActive = index === current;
  return (
    <div className="wz-dot-wrap">
      <div className={`wz-dot ${isDone ? "done" : ""} ${isActive ? "active" : ""}`}>
        {isDone ? "✓" : index}
      </div>
      <span className={`wz-dot-label ${isActive ? "active" : ""}`}>
        {STEP_LABELS[index - 1]}
      </span>
    </div>
  );
}

function Tag({ children }) {
  return <span className="wz-tag">{children}</span>;
}

function NavRow({ onBack, onNext, nextDisabled, nextLabel }) {
  return (
    <div className="wz-nav">
      {onBack ? (
        <button className="wz-btn" onClick={onBack}>← Kembali</button>
      ) : (
        <span />
      )}
      <button className="wz-btn primary" onClick={onNext} disabled={nextDisabled}>
        {nextLabel ?? "Lanjut →"}
      </button>
    </div>
  );
}

// ─── Main Component ───────────────────────────────────────────────────────────

export default function SchoolWizard() {
  const [step, setStep] = useState(1);
  const [state, setState] = useState({
    system: "",
    units: [],
    portals: [],
    features: [],
    profile: {
      nama: "", skPendirian: "", npsn: "", alamat: "",
      kota: "", provinsi: "", telp: "", email: "", website: "",
    },
  });

  const units = state.system === "sekolah" ? SEKOLAH_UNITS : KAMPUS_UNITS;
  const progressWidth = [0, 0, 20, 40, 60, 80, 100][step] + "%";

  const profileComplete =
    state.profile.nama.trim() &&
    state.profile.alamat.trim() &&
    state.profile.kota.trim() &&
    state.profile.provinsi;

  // ── Step 1: System ──
  function Step1() {
    return (
      <div className="wz-panel">
        <h2 className="wz-title">Saya ingin membuat sistem untuk</h2>
        <p className="wz-sub">Pilih satu jenis institusi yang ingin Anda kelola</p>
        <div className="wz-grid-1">
          {["sekolah", "kampus"].map((s) => (
            <label
              key={s}
              className={`wz-option-card ${state.system === s ? "selected" : ""}`}
              onClick={() => setState((p) => ({ ...p, system: s, units: [] }))}
            >
              <input type="radio" name="system" value={s} checked={state.system === s} readOnly />
              <span className="wz-option-icon">{s === "sekolah" ? "🏫" : "🏛️"}</span>
              <div>
                <div className="wz-option-label">
                  {s === "sekolah" ? "Sistem Sekolah" : "Sistem Kampus / Perguruan Tinggi"}
                </div>
                <div className="wz-option-desc">
                  {s === "sekolah"
                    ? "PAUD, TK, SD, SMP, SMA — kelola satu atau beberapa jenjang sekaligus"
                    : "Universitas, Politeknik, Akademi — kelola fakultas dan program studi"}
                </div>
              </div>
            </label>
          ))}
        </div>
        <NavRow onNext={() => setStep(2)} nextDisabled={!state.system} />
      </div>
    );
  }

  // ── Step 2: Units ──
  function Step2() {
    return (
      <div className="wz-panel">
        <h2 className="wz-title">
          {state.system === "sekolah" ? "Pilih jenjang sekolah" : "Pilih fakultas / program"}
        </h2>
        <p className="wz-sub">Bisa pilih lebih dari satu</p>
        <div className="wz-grid-1">
          {units.map((u) => (
            <label
              key={u.v}
              className={`wz-option-card ${state.units.includes(u.v) ? "selected" : ""}`}
              onClick={() => setState((p) => ({ ...p, units: toggle(p.units, u.v) }))}
            >
              <input type="checkbox" checked={state.units.includes(u.v)} readOnly />
              <span className="wz-option-icon">{u.icon}</span>
              <div>
                <div className="wz-option-label">{u.l}</div>
                <div className="wz-option-desc">{u.d}</div>
              </div>
            </label>
          ))}
        </div>
        <NavRow
          onBack={() => setStep(1)}
          onNext={() => setStep(3)}
          nextDisabled={state.units.length === 0}
        />
      </div>
    );
  }

  // ── Step 3: Profile ──
  function Step3() {
    const p = state.profile;
    const handleChange = (key) => (e) =>
      setState((prev) => ({ ...prev, profile: { ...prev.profile, [key]: e.target.value } }));

    return (
      <div className="wz-panel">
        <h2 className="wz-title">Profil Yayasan / Institusi</h2>
        <p className="wz-sub">Lengkapi informasi resmi lembaga Anda</p>
        <div className="wz-form-grid">
          <div className="wz-field full">
            <label>Nama Yayasan / Institusi <span className="req">*</span></label>
            <input placeholder="cth. Yayasan Pendidikan Nusantara" value={p.nama} onChange={handleChange("nama")} />
          </div>
          <div className="wz-field">
            <label>No. SK Pendirian</label>
            <input placeholder="cth. 123/SK/2010" value={p.skPendirian} onChange={handleChange("skPendirian")} />
          </div>
          <div className="wz-field">
            <label>NPSN / NIS / NIPT</label>
            <input placeholder="Nomor pokok sekolah/PT" value={p.npsn} onChange={handleChange("npsn")} />
          </div>
          <div className="wz-field full">
            <label>Alamat Lengkap <span className="req">*</span></label>
            <input placeholder="Jalan, Kelurahan, Kecamatan" value={p.alamat} onChange={handleChange("alamat")} />
          </div>
          <div className="wz-field">
            <label>Kota / Kabupaten <span className="req">*</span></label>
            <input placeholder="cth. Kota Bandung" value={p.kota} onChange={handleChange("kota")} />
          </div>
          <div className="wz-field">
            <label>Provinsi <span className="req">*</span></label>
            <select value={p.provinsi} onChange={handleChange("provinsi")}>
              <option value="">Pilih provinsi…</option>
              {PROVINCES.map((prov) => <option key={prov}>{prov}</option>)}
            </select>
          </div>
          <div className="wz-field">
            <label>No. Telepon</label>
            <input placeholder="021-XXXXXXXX" value={p.telp} onChange={handleChange("telp")} />
          </div>
          <div className="wz-field">
            <label>Email Resmi</label>
            <input type="email" placeholder="admin@lembaga.sch.id" value={p.email} onChange={handleChange("email")} />
          </div>
          <div className="wz-field full">
            <label>Website (opsional)</label>
            <input placeholder="https://www.lembaga.sch.id" value={p.website} onChange={handleChange("website")} />
          </div>
        </div>
        <NavRow
          onBack={() => setStep(2)}
          onNext={() => setStep(4)}
          nextDisabled={!profileComplete}
        />
      </div>
    );
  }

  // ── Step 4: Portals ──
  function Step4() {
    return (
      <div className="wz-panel">
        <h2 className="wz-title">Aktifkan portal yang dibutuhkan</h2>
        <p className="wz-sub">Pilih portal yang akan digunakan di sistem Anda</p>
        <div className="wz-portal-grid">
          {PORTAL_OPTIONS.map((po) => {
            const title = po.title ?? (state.system === "sekolah" ? po.titleSekolah : po.titleKampus) ?? "";
            return (
              <div
                key={po.v}
                className={`wz-portal-card ${state.portals.includes(po.v) ? "selected" : ""}`}
                onClick={() => setState((p) => ({ ...p, portals: toggle(p.portals, po.v) }))}
              >
                <div className="wz-picon">{po.icon}</div>
                <div className="wz-ptitle">{title}</div>
                <div className="wz-pdesc">{po.desc}</div>
              </div>
            );
          })}
        </div>
        <NavRow
          onBack={() => setStep(3)}
          onNext={() => setStep(5)}
          nextDisabled={state.portals.length === 0}
        />
      </div>
    );
  }

  // ── Step 5: Features ──
  function Step5() {
    return (
      <div className="wz-panel">
        <h2 className="wz-title">Pilih fitur yang diperlukan</h2>
        <p className="wz-sub">Aktifkan modul sesuai kebutuhan — bisa ditambah kapan saja</p>
        <div className="wz-feature-grid">
          {ALL_FEATURES.map((f) => (
            <div
              key={f.v}
              className={`wz-feature-chip ${state.features.includes(f.v) ? "selected" : ""}`}
              onClick={() => setState((p) => ({ ...p, features: toggle(p.features, f.v) }))}
            >
              <span className="wz-ficon">{f.icon}</span>
              {f.l}
            </div>
          ))}
        </div>
        <NavRow
          onBack={() => setStep(4)}
          onNext={() => setStep(6)}
          nextLabel="Lihat Ringkasan →"
        />
      </div>
    );
  }

  // ── Step 6: Summary ──
  function Step6() {
    const selUnits = units.filter((u) => state.units.includes(u.v));
    const selPortals = PORTAL_OPTIONS.filter((po) => state.portals.includes(po.v)).map(
      (po) => po.title ?? (state.system === "sekolah" ? po.titleSekolah : po.titleKampus) ?? ""
    );
    const selFeatures = ALL_FEATURES.filter((f) => state.features.includes(f.v));

    const sections = [
      { label: "Jenis Sistem", tags: [state.system === "sekolah" ? "Sistem Sekolah" : "Sistem Kampus"] },
      {
        label: state.system === "sekolah" ? "Jenjang Sekolah" : "Fakultas / Program",
        tags: selUnits.map((u) => u.l),
      },
      { label: "Institusi", tags: [state.profile.nama, state.profile.kota].filter(Boolean) },
      { label: "Portal Aktif", tags: selPortals },
      { label: "Fitur Terpilih", tags: selFeatures.map((f) => f.l) },
    ];

    return (
      <div className="wz-panel">
        <h2 className="wz-title">Konfigurasi Anda siap! 🎉</h2>
        <p className="wz-sub">Berikut ringkasan sistem yang akan dibangun untuk Anda</p>

        {sections.map((section, i) => (
          <div key={i}>
            <div className="wz-divider" />
            <div className="wz-summary-section">
              <div className="wz-sum-label">{section.label}</div>
              <div className="wz-sum-tags">
                {section.tags.length > 0
                  ? section.tags.map((t) => <Tag key={t}>{t}</Tag>)
                  : <span className="wz-empty">Belum dipilih</span>}
              </div>
            </div>
          </div>
        ))}

        <div className="wz-cta-row">
          <div className="wz-cta-card demo">
            <div className="wz-cta-icon">🎬</div>
            <div className="wz-cta-title">Coba Demo Gratis</div>
            <div className="wz-cta-desc">Akses langsung sistem sesuai konfigurasi Anda, tanpa registrasi kartu kredit</div>
            <button className="wz-cta-btn demo-btn">Mulai Demo →</button>
          </div>
          <div className="wz-cta-card sub">
            <div className="wz-cta-icon">🚀</div>
            <div className="wz-cta-title">Langganan Sekarang</div>
            <div className="wz-cta-desc">Pilih paket yang sesuai, langsung aktif dalam 1x24 jam kerja</div>
            <button className="wz-cta-btn sub-btn">Lihat Paket →</button>
          </div>
        </div>

        <div style={{ marginTop: "1rem", textAlign: "center" }}>
          <button className="wz-btn" onClick={() => setStep(5)}>← Kembali Edit</button>
        </div>
      </div>
    );
  }

  // ── Render ──
  const StepComponents = [null, <Step1 />, <Step2 />, <Step3 />, <Step4 />, <Step5 />, <Step6 />];

  return (
    <>
      <style>{`
        @import url('https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;500;600;700&display=swap');

        .wz-root { font-family: 'Plus Jakarta Sans', sans-serif; max-width: 680px; margin: 0 auto; padding: 2rem 1rem; }

        /* Progress */
        .wz-progress { display: flex; align-items: flex-start; gap: 0; margin-bottom: 2rem; position: relative; }
        .wz-progress-track { position: absolute; top: 17px; left: 18px; right: 18px; height: 2px; background: #e5e7eb; z-index: 0; }
        .wz-progress-fill { position: absolute; top: 17px; left: 18px; height: 2px; background: #2563eb; z-index: 1; transition: width 0.5s cubic-bezier(.4,0,.2,1); }
        .wz-dot-wrap { flex: 1; display: flex; flex-direction: column; align-items: center; gap: 6px; position: relative; z-index: 2; }
        .wz-dot { width: 36px; height: 36px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 13px; font-weight: 600; border: 2px solid #e5e7eb; background: #fff; color: #9ca3af; transition: all 0.3s ease; }
        .wz-dot.active { border-color: #2563eb; background: #2563eb; color: #fff; box-shadow: 0 0 0 4px #dbeafe; }
        .wz-dot.done { border-color: #16a34a; background: #16a34a; color: #fff; }
        .wz-dot-label { font-size: 11px; color: #9ca3af; white-space: nowrap; }
        .wz-dot-label.active { color: #2563eb; font-weight: 600; }

        /* Panel */
        .wz-panel { background: #fff; border-radius: 16px; border: 1px solid #e5e7eb; padding: 1.75rem; animation: wzFade 0.3s ease; box-shadow: 0 1px 3px rgba(0,0,0,.06); }
        @keyframes wzFade { from { opacity:0; transform: translateY(8px); } to { opacity:1; transform: translateY(0); } }
        .wz-title { font-size: 19px; font-weight: 700; color: #111827; margin-bottom: 4px; letter-spacing: -0.3px; }
        .wz-sub { font-size: 14px; color: #6b7280; margin-bottom: 1.5rem; }

        /* Options */
        .wz-grid-1 { display: grid; gap: 10px; }
        .wz-option-card { display: flex; align-items: flex-start; gap: 14px; padding: 14px 16px; border-radius: 12px; border: 1.5px solid #e5e7eb; cursor: pointer; transition: all 0.2s ease; background: #fafafa; }
        .wz-option-card:hover { border-color: #2563eb; background: #eff6ff; }
        .wz-option-card.selected { border-color: #2563eb; background: #eff6ff; }
        .wz-option-card input { accent-color: #2563eb; width: 17px; height: 17px; flex-shrink: 0; margin-top: 3px; }
        .wz-option-icon { font-size: 22px; flex-shrink: 0; margin-top: 1px; }
        .wz-option-label { font-size: 14px; font-weight: 600; color: #111827; line-height: 1.3; }
        .wz-option-desc { font-size: 12px; color: #6b7280; margin-top: 3px; }

        /* Form */
        .wz-form-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; }
        .wz-field { display: flex; flex-direction: column; }
        .wz-field.full { grid-column: 1 / -1; }
        .wz-field label { font-size: 12px; font-weight: 600; color: #374151; margin-bottom: 5px; }
        .wz-field .req { color: #ef4444; }
        .wz-field input, .wz-field select { width: 100%; padding: 9px 12px; font-size: 14px; font-family: inherit; border: 1.5px solid #e5e7eb; border-radius: 8px; background: #fafafa; color: #111827; outline: none; transition: border-color 0.2s; }
        .wz-field input:focus, .wz-field select:focus { border-color: #2563eb; background: #fff; }

        /* Portals */
        .wz-portal-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 10px; }
        .wz-portal-card { padding: 16px; border-radius: 12px; border: 1.5px solid #e5e7eb; cursor: pointer; transition: all 0.2s ease; background: #fafafa; }
        .wz-portal-card:hover { border-color: #2563eb; background: #eff6ff; }
        .wz-portal-card.selected { border-color: #2563eb; background: #eff6ff; }
        .wz-picon { font-size: 24px; margin-bottom: 8px; }
        .wz-ptitle { font-size: 13px; font-weight: 600; color: #111827; }
        .wz-pdesc { font-size: 11px; color: #6b7280; margin-top: 3px; }

        /* Features */
        .wz-feature-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 8px; }
        .wz-feature-chip { display: flex; align-items: center; gap: 10px; padding: 11px 14px; border-radius: 10px; border: 1.5px solid #e5e7eb; cursor: pointer; font-size: 13px; font-weight: 500; transition: all 0.2s; background: #fafafa; color: #374151; }
        .wz-feature-chip:hover { border-color: #2563eb; background: #eff6ff; }
        .wz-feature-chip.selected { border-color: #2563eb; background: #eff6ff; color: #1d4ed8; }
        .wz-ficon { font-size: 17px; flex-shrink: 0; }

        /* Nav */
        .wz-nav { display: flex; justify-content: space-between; align-items: center; margin-top: 1.5rem; }
        .wz-btn { padding: 10px 22px; border-radius: 10px; font-size: 14px; font-weight: 600; cursor: pointer; border: 1.5px solid #e5e7eb; background: #fff; color: #374151; transition: all 0.2s; font-family: inherit; }
        .wz-btn:hover { background: #f9fafb; }
        .wz-btn.primary { background: #2563eb; color: #fff; border-color: #2563eb; }
        .wz-btn.primary:hover { background: #1d4ed8; border-color: #1d4ed8; }
        .wz-btn.primary:disabled { background: #93c5fd; border-color: #93c5fd; cursor: not-allowed; }

        /* Summary */
        .wz-divider { height: 1px; background: #f3f4f6; margin: 0.875rem 0; }
        .wz-sum-label { font-size: 11px; color: #9ca3af; text-transform: uppercase; letter-spacing: .06em; margin-bottom: 8px; font-weight: 600; }
        .wz-sum-tags { display: flex; flex-wrap: wrap; gap: 6px; }
        .wz-tag { padding: 4px 11px; border-radius: 100px; font-size: 12px; background: #eff6ff; color: #2563eb; font-weight: 600; border: 1px solid #bfdbfe; }
        .wz-empty { font-size: 13px; color: #9ca3af; }

        /* CTA */
        .wz-cta-row { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; margin-top: 1.5rem; }
        .wz-cta-card { padding: 1.25rem; border-radius: 14px; border: 1.5px solid #e5e7eb; text-align: center; }
        .wz-cta-card.demo { border-color: #7c3aed; background: #faf5ff; }
        .wz-cta-card.sub { border-color: #16a34a; background: #f0fdf4; }
        .wz-cta-icon { font-size: 28px; margin-bottom: 8px; }
        .wz-cta-title { font-size: 15px; font-weight: 700; color: #111827; }
        .wz-cta-desc { font-size: 12px; color: #6b7280; margin-top: 4px; line-height: 1.5; }
        .wz-cta-btn { margin-top: 12px; padding: 9px 0; width: 100%; border-radius: 10px; font-size: 13px; font-weight: 600; background: transparent; cursor: pointer; font-family: inherit; transition: all 0.2s; }
        .wz-cta-btn.demo-btn { border: 1.5px solid #7c3aed; color: #7c3aed; }
        .wz-cta-btn.demo-btn:hover { background: #7c3aed; color: #fff; }
        .wz-cta-btn.sub-btn { border: 1.5px solid #16a34a; color: #16a34a; }
        .wz-cta-btn.sub-btn:hover { background: #16a34a; color: #fff; }

        @media (max-width: 520px) {
          .wz-form-grid { grid-template-columns: 1fr; }
          .wz-field.full { grid-column: 1; }
          .wz-feature-grid { grid-template-columns: 1fr; }
          .wz-cta-row { grid-template-columns: 1fr; }
        }
      `}</style>

      <div className="wz-root">
        {/* Progress Bar */}
        <div className="wz-progress">
          <div className="wz-progress-track" />
          <div className="wz-progress-fill" style={{ width: progressWidth }} />
          {[1, 2, 3, 4, 5, 6].map((i) => (
            <StepDot key={i} index={i} current={step} />
          ))}
        </div>

        {/* Steps */}
        {StepComponents[step]}
      </div>
    </>
  );
}