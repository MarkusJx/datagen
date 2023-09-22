import { useEffect, useRef, useState } from 'react';
import { ModuleThread, spawn, Thread, Worker } from 'threads';
import { wasmSupported, webWorkersSupported } from '../util/util';
import type {
  GenerateDataWebWorker,
  GenerateWorkerProgress,
} from '../worker/demo';

type GenerateWorker = {
  generateRandomDataWebWorker: GenerateDataWebWorker;
};

type DemoWorkerCallback = (
  schema: any,
  setGenerating: (generating: boolean) => void,
  setGenerated: (data: string) => void,
  isParsed: boolean,
  progressCallback?: (progress: number) => void
) => Promise<void>;
type DemoWorker = {
  workerInitialized: boolean;
  generateRandomData: DemoWorkerCallback;
};
type LoadCallback = () => void | Promise<void>;

const useDemoWorker = (onUnsupported?: LoadCallback): DemoWorker => {
  const worker = useRef<ModuleThread<GenerateWorker>>();
  const [initialized, setInitialized] = useState(false);

  useEffect(() => {
    if (!webWorkersSupported() || !wasmSupported()) {
      if (onUnsupported) {
        onUnsupported();
      }
      return;
    }

    if (!worker.current) {
      const load = async () => {
        worker.current = await spawn<GenerateWorker>(
          new Worker(
            new URL('../worker/demo', import.meta.url) as unknown as string
          )
        );

        setInitialized(true);
      };

      load().catch(console.error);
    }

    return () => {
      if (worker.current) {
        Thread.terminate(worker.current).catch(console.error);
      }
    };
  }, []);

  return {
    workerInitialized: initialized,
    generateRandomData: async (
      schema,
      setGenerating,
      setGenerated,
      isParsed,
      progressCallback
    ) => {
      if (!worker.current) throw new Error('Worker not loaded');

      try {
        setGenerating(true);
        const res = worker.current.generateRandomDataWebWorker(
          isParsed ? schema : JSON.parse(schema),
          !!progressCallback
        );

        if (progressCallback) {
          res.subscribe({
            next(value: GenerateWorkerProgress) {
              if (value.data) {
                progressCallback(1);
                setGenerated(value.data);
              } else {
                progressCallback(value.progress);
              }
            },
            complete() {
              setGenerating(false);
            },
            error(e) {
              console.error(e);
              setGenerated('Error: ' + e.message);
              setGenerating(false);
            },
          });
        } else {
          try {
            setGenerated((await res) as string);
          } finally {
            setGenerating(false);
          }
        }
      } catch (e: any) {
        console.error(e);
        setGenerated('Error: ' + e.message);
      }
    },
  };
};

export default useDemoWorker;
