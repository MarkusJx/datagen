import { useEffect, useState } from 'react';

type LocalStorage<T, D extends boolean> = [
  initialized: boolean,
  value: D extends true ? T : T | null,
  setValue: (value: T) => void,
  clear: () => void,
];

export default function useLocalStorage<T>(key: string): LocalStorage<T, false>;
export default function useLocalStorage<T>(
  key: string,
  defaultValue: T
): LocalStorage<T, true>;
export default function useLocalStorage<T>(
  key: string,
  defaultValue?: T
): LocalStorage<T, boolean> {
  const [initialized, setInitialized] = useState(false);
  const [value, setValue] = useState<T | null>(defaultValue ?? null);

  useEffect(() => {
    const item = localStorage.getItem(key);
    setValue(item ? JSON.parse(item) : defaultValue ?? null);
    setInitialized(true);
  }, []);

  return [
    initialized,
    value,
    (value) => {
      localStorage.setItem(key, JSON.stringify(value));
      setValue(value);
    },
    () => {
      localStorage.removeItem(key);
      setValue(defaultValue ?? null);
    },
  ];
}
