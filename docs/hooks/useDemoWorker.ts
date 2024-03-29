import { useEffect, useRef, useState } from 'react';
import { ModuleThread, spawn, Thread, Worker } from 'threads';
import { wasmSupported, webWorkersSupported } from '../util/util';
import type {
  GenerateDataWebWorker,
  GenerateWorkerProgress,
} from '../worker/demo';
import { Schema } from 'datagen-rs-types';

type GenerateWorker = {
  generateRandomDataWebWorker: GenerateDataWebWorker;
};

type DemoWorkerCallback = (
  schema: Schema | string,
  setGenerating: (generating: boolean) => void,
  setGenerated: (data: string) => void,
  isParsed: boolean,
  progressCallback?: (progress: number) => void
) => Promise<void>;

export type DemoWorker = {
  workerInitialized: boolean;
  workerSupported: boolean;
  workerError?: string;
  generateRandomData: DemoWorkerCallback;
};

const useDemoWorker = (): DemoWorker => {
  const worker = useRef<ModuleThread<GenerateWorker>>();
  const [initialized, setInitialized] = useState(!!worker.current);
  const [supported, setSupported] = useState(true);
  const [error, setError] = useState<string>();

  useEffect(() => {
    if (!webWorkersSupported() || !wasmSupported()) {
      setSupported(false);
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

      load().catch((e) => {
        console.error('Error initializing worker', e);
        setError(e);
      });
    } else {
      setInitialized(true);
    }

    return () => {
      if (worker.current) {
        Thread.terminate(worker.current).catch(console.error);
      }
    };
  }, []);

  return {
    workerInitialized: initialized,
    workerSupported: supported,
    workerError: error,
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
          isParsed ? schema : JSON.parse(schema as string),
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
              console.error('Failed to generate data', e);
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
        console.error('Failed to generate data', e);
        setGenerated('Error: ' + e.message);
        setGenerating(false);
      }
    },
  };
};

export default useDemoWorker;
