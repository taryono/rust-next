export default function StepFeature({
  state,
  setState,
  setStep,
  ALL_FEATURES,
  toggle,
}) {
  return (
    <div className="wz-panel">
      <h2 className="wz-title">Fitur</h2>

      <div className="wz-feature-grid">
        {ALL_FEATURES.map((f) => (
          <div
            key={f.v}
            className={`wz-feature-chip ${
              state.features.includes(f.v) ? "selected" : ""
            }`}
            onClick={() =>
              setState((p) => ({
                ...p,
                features: toggle(p.features, f.v),
              }))
            }
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