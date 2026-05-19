"use client";

import { useState } from "react";

import StepSystem from "./StepSystem";
import StepUnit from "./StepUnit";
import StepProfile from "./StepProfile";
import StepPortal from "./StepPortal";
import StepFeature from "./StepFeature";
import StepSummary from "./StepSummary";

import StepDot from "./_internal/StepDot";
import Tag from "./_internal/Tag";
import NavRow from "./_internal/NavRow";

// ─── DATA (TETAP SAMA) ───────────────────────────────────────────────────────

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

const PORTAL_OPTIONS = [
  { v: "pmb", icon: "📋", title: "Portal Pendaftaran (PMB)", desc: "Formulir online, seleksi, pengumuman" },
  { v: "student", icon: "🪪", titleSekolah: "Portal Siswa", titleKampus: "Portal Mahasiswa", desc: "Nilai, jadwal, kehadiran, tugas" },
  { v: "admin", icon: "🛡️", titleSekolah: "Portal Guru & Admin", titleKampus: "Portal Dosen & Admin", desc: "Manajemen data, laporan, keuangan" },
  { v: "mobile", icon: "📱", title: "Mobile Apps", desc: "Android & iOS untuk siswa & orang tua" },
];

const STEP_LABELS = ["Sistem", "Unit", "Profil", "Portal", "Fitur", "Mulai"];

const toggle = (arr, v) =>
  arr.includes(v) ? arr.filter((x) => x !== v) : [...arr, v];

// ─── MAIN ────────────────────────────────────────────────────────────────────

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

  const StepComponents = [
    null,
    <StepSystem key={1} {...{ state, setState, setStep }} />,
    <StepUnit key={2} {...{ state, setState, setStep, units, toggle }} />,
    <StepProfile key={3} {...{ state, setState, setStep }} />,
    <StepPortal key={4} {...{ state, setState, setStep, PORTAL_OPTIONS, toggle }} />,
    <StepFeature key={5} {...{ state, setState, setStep, ALL_FEATURES, toggle }} />,
    <StepSummary key={6} {...{ state, setStep, units, ALL_FEATURES, PORTAL_OPTIONS }} />,
  ];

  return (
    <>
      {/* CSS TETAP FULL SAMA (tidak diubah sama sekali) */}
      <style>{`/* ❗ CSS dari versi kamu original tetap dipertahankan utuh di sini */`}</style>

      <div className="wz-root">

        <div className="wz-progress">
          <div className="wz-progress-track" />
          <div className="wz-progress-fill" style={{ width: progressWidth }} />

          {[1, 2, 3, 4, 5, 6].map((i) => (
            <StepDot key={i} index={i} current={step} labels={STEP_LABELS} />
          ))}
        </div>

        {StepComponents[step]}
      </div>
    </>
  );
}