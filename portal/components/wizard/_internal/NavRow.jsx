export default function NavRow({ onBack, onNext, nextDisabled, nextLabel }) {
  return (
    <div className="wz-nav">
      {onBack ? (
        <button className="wz-btn" onClick={onBack}>
          ← Kembali
        </button>
      ) : (
        <span />
      )}

      <button
        className="wz-btn primary"
        onClick={onNext}
        disabled={nextDisabled}
      >
        {nextLabel ?? "Lanjut →"}
      </button>
    </div>
  );
}