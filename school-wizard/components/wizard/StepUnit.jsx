export default function StepUnit({ state, setState, setStep, units, toggle }) {
  return (
    <div className="wz-panel">
      <h2 className="wz-title">
        {state.system === "sekolah"
          ? "Pilih jenjang sekolah"
          : "Pilih fakultas"}
      </h2>

      <div className="wz-grid-1">
        {units.map((u) => (
          <label
            key={u.v}
            className={`wz-option-card ${
              state.units.includes(u.v) ? "selected" : ""
            }`}
            onClick={() =>
              setState((p) => ({
                ...p,
                units: toggle(p.units, u.v),
              }))
            }
          >
            <input
              type="checkbox"
              checked={state.units.includes(u.v)}
              readOnly
            />
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
        nextDisabled={!state.units.length}
      />
    </div>
  );
}