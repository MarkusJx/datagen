import init, { GenerateProgress, generateRandomData } from 'datagen-rs-wasm';
import { expose } from 'threads/worker';
import { Observable, SubscriptionObserver } from 'observable-fns';

export type GenerateWorkerProgress = {
  progress: number;
  data: string | null;
};
export type GenerateDataWebWorker = typeof generateRandomDataWebWorker;
const SEND_INTERVAL = 250;

const generateWithProgress = async (
  subscriber: SubscriptionObserver<GenerateWorkerProgress>,
  data: any
) => {
  await init();
  subscriber.next({
    progress: 0,
    data: null,
  });

  let lastSend = new Date().getTime();
  const result = generateRandomData(data, (progress: GenerateProgress) => {
    try {
      if (new Date().getTime() - lastSend >= SEND_INTERVAL) {
        lastSend = new Date().getTime();
        subscriber.next({
          progress: progress.current / progress.total,
          data: null,
        });
      }
    } finally {
      progress?.free();
    }
  });

  subscriber.next({
    progress: 1,
    data: result,
  });
  subscriber.complete();
};

const generateWithoutProgress = async (data: any) => {
  await init();
  return generateRandomData(data);
};

function generateRandomDataWebWorker(
  data: any,
  progress: boolean
): Observable<GenerateWorkerProgress> | Promise<string> {
  if (progress) {
    return new Observable<GenerateWorkerProgress>((subscriber) => {
      generateWithProgress(subscriber, data).catch(
        subscriber.error.bind(subscriber)
      );
    });
  } else {
    return generateWithoutProgress(data);
  }
}

expose({ generateRandomDataWebWorker });
