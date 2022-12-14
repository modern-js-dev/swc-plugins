import { cloneElement } from 'react';

export default function Composer(props) {
  return renderRecursive(props.children, props.components);
}

function renderRecursive(render, remaining, results) {
  results = results || [];

  if (!remaining[0]) {
    return render(results);
  }

  function nextRender(value) {
    return renderRecursive(render, remaining.slice(1), results.concat([
      value
    ]));
  }

  return typeof remaining[0] === 'function' ? remaining[0]({
    results,
    render: nextRender
  }) : /*#__PURE__*/ cloneElement(remaining[0], {
    children: nextRender
  });
}
