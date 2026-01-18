export default function TreeSelect({ node, level = 0 }) {
  return (
    <>
      <option value={node.id}>
        {'—'.repeat(level)} {node.name}
      </option>

      {node.children?.map(child => (
        <TreeSelect key={child.id} node={child} level={level + 1} />
      ))}
    </>
  );
}