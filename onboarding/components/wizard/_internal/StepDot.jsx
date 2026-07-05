export default function StepDot({ index, current, labels }) {
  const isDone = index < current;
  const isActive = index === current;

  return (
    <div className="wz-dot-wrap">
      <div className={`wz-dot ${isDone ? "done" : ""} ${isActive ? "active" : ""}`}>
        {isDone ? "✓" : index}
      </div>
      <span className={`wz-dot-label ${isActive ? "active" : ""}`}>
        {labels[index - 1]}
      </span>
    </div>
  );
}