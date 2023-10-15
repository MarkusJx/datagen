import React from 'react';
import { createRoot } from 'react-dom/client';

export const renderChild = <K extends keyof HTMLElementTagNameMap>(
  children: React.ReactNode,
  tagName: K,
  classNames: string[] = []
): HTMLElementTagNameMap[K] => {
  const container = document.createElement(tagName);
  createRoot(container).render(children);
  container.classList.add(...classNames);

  return container;
};
