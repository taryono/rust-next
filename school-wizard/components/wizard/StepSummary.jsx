import Tag from "./_internal/Tag";

export default function StepSummary({
  state,
  setStep,
  units,
  ALL_FEATURES,
  PORTAL_OPTIONS,
}) {
  const selUnits = units.filter((u) => state.units.includes(u.v));
  const selFeatures = ALL_FEATURES.filter((f) =>
    state.features.includes(f.v)
  );
  const selPortals = PORTAL_OPTIONS.filter((p) =>
    state.portals.includes(p.v)
  );

  return (
    <div className="wz-panel">
      <h2 className="wz-title">Konfigurasi Anda siap! 🎉</h2>
      <p className="wz-sub">Ringkasan sistem</p>

      <div className="wz-summary-section">
        <div className="wz-sum-label">Unit</div>
        <div className="wz-sum-tags">
          {selUnits.map((u) => (
            <Tag key={u.v}>{u.l}</Tag>
          ))}
        </div>
      </div>

      <div className="wz-summary-section">
        <div className="wz-sum-label">Fitur</div>
        <div className="wz-sum-tags">
          {selFeatures.map((f) => (
            <Tag key={f.v}>{f.l}</Tag>
          ))}
        </div>
      </div>

      <NavRow
        onBack={() => setStep(5)}
        onNext={() => alert("Finish")}
        nextLabel="Finish"
      />
    </div>
  );
}