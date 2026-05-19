export default function StepProfile({ state, setState, setStep }) {
  const p = state.profile;

  const handle = (key) => (e) =>
    setState((prev) => ({
      ...prev,
      profile: { ...prev.profile, [key]: e.target.value },
    }));

  return (
    <div className="wz-panel">
      <h2 className="wz-title">Profil Yayasan / Institusi</h2>
      <p className="wz-sub">Lengkapi informasi resmi lembaga Anda</p>

      <div className="wz-form-grid">
        <div className="wz-field full">
          <label>Nama *</label>
          <input value={p.nama} onChange={handle("nama")} />
        </div>

        <div className="wz-field full">
          <label>Alamat *</label>
          <input value={p.alamat} onChange={handle("alamat")} />
        </div>

        <div className="wz-field">
          <label>Kota *</label>
          <input value={p.kota} onChange={handle("kota")} />
        </div>

        <div className="wz-field">
          <label>Provinsi *</label>
          <input value={p.provinsi} onChange={handle("provinsi")} />
        </div>
      </div>

      <NavRow
        onBack={() => setStep(2)}
        onNext={() => setStep(4)}
        nextDisabled={!p.nama || !p.alamat}
      />
    </div>
  );
}