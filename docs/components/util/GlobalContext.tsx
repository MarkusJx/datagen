import React, { createContext, useContext } from 'react';
import useDemoWorker, { DemoWorker } from '../../hooks/useDemoWorker';

interface GlobalContextIf extends DemoWorker {}

const Context = createContext<GlobalContextIf>({
  workerInitialized: false,
  workerSupported: false,
  generateRandomData: () => Promise.resolve(),
});

export const useGlobalContext = () => {
  return useContext(Context);
};

const GlobalContext: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const {
    workerError,
    workerInitialized,
    workerSupported,
    generateRandomData,
  } = useDemoWorker();

  return (
    <Context.Provider
      value={{
        workerError,
        workerInitialized,
        workerSupported,
        generateRandomData,
      }}
    >
      {children}
    </Context.Provider>
  );
};

export default GlobalContext;
