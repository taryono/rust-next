export default function StepSystem({ state, setState, setStep }) {
  return (
    <div className="wz-panel">
      <h2 className="wz-title">Saya ingin membuat sistem untuk</h2>
      <p className="wz-sub">Pilih satu jenis institusi yang ingin Anda kelola</p>

      <div className="wz-grid-1">
        {["sekolah", "kampus"].map((s) => (
          <label
            key={s}
            className={`wz-option-card ${state.system === s ? "selected" : ""}`}
            onClick={() =>
              setState((p) => ({ ...p, system: s, units: [] }))
            }
          >
            <input type="radio" checked={state.system === s} readOnly />
            <span className="wz-option-icon">
              {s === "sekolah" ? "🏫" : "🏛️"}
            </span>
            <div>
              <div className="wz-option-label">
                {s === "sekolah"
                  ? "Sistem Sekolah"
                  : "Sistem Kampus / Perguruan Tinggi"}
              </div>
              <div className="wz-option-desc">
                {s === "sekolah"
                  ? "PAUD, TK, SD, SMP, SMA"
                  : "Universitas, Politeknik, Akademi"}
              </div>
            </div>
          </label>
        ))}
      </div>

      <div className="wz-nav">
        <span />
        <button
          className="wz-btn primary"
          onClick={() => setStep(2)}
          disabled={!state.system}
        >
          Lanjut →
        </button>
      </div>
    </div>
  );
}