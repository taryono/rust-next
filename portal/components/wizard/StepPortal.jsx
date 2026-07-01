export default function StepPortal({
  state,
  setState,
  setStep,
  PORTAL_OPTIONS,
  toggle,
}) {
  return (
    <div className="wz-panel">
      <h2 className="wz-title">Portal</h2>

      <div className="wz-portal-grid">
        {PORTAL_OPTIONS.map((po) => (
          <div
            key={po.v}
            className={`wz-portal-card ${
              state.portals.includes(po.v) ? "selected" : ""
            }`}
            onClick={() =>
              setState((p) => ({
                ...p,
                portals: toggle(p.portals, po.v),
              }))
            }
          >
            <div className="wz-picon">{po.icon}</div>
            <div className="wz-ptitle">{po.title}</div>
            <div className="wz-pdesc">{po.desc}</div>
          </div>
        ))}
      </div>

      <NavRow
        onBack={() => setStep(3)}
        onNext={() => setStep(5)}
        nextDisabled={!state.portals.length}
      />
    </div>
  );
}