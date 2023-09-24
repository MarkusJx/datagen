import yargs, { BuilderCallback, ArgumentsCamelCase } from 'yargs';
import { Presets, SingleBar } from 'cli-progress';
import { generateRandomData } from '@datagen-rs/node';
import fs from 'node:fs/promises';
import chalk from 'chalk';
import { Schema } from '@datagen-rs/types';

type YargsHandler<T> = (args: ArgumentsCamelCase<T>) => Promise<void>;
interface Args {
  schema: string;
  output?: string;
}

const builder: BuilderCallback<{}, Args> = (command) => {
  command
    .positional('schema', {
      describe: 'The schema file',
      type: 'string',
    })
    .option('output', {
      type: 'string',
      description: 'The output file',
    })
    .showHelpOnFail(true);
};

const handler: YargsHandler<Args> = async ({ schema, output }) => {
  let bar: SingleBar | null = null;

  const read = await fs.readFile(schema, 'utf-8');
  const res = await generateRandomData(
    JSON.parse(read) as Schema,
    (progress) => {
      if (!bar) {
        console.log(`Generating ${chalk.cyanBright(progress.total)} records`);
        bar = new SingleBar(
          {
            format:
              'Generating records [{duration_formatted}] |{bar}| {percentage}% || {value}/{total}',
          },
          Presets.shades_classic
        );
        bar.start(progress.total, progress.current);
      }

      bar.update(progress.current);
    }
  );

  // @ts-ignore
  bar?.stop();

  if (output) {
    await fs.writeFile(output, res, 'utf-8');
  } else {
    console.log(res);
  }
};

yargs(process.argv.slice(2))
  .command('* <schema> [output]', false, builder, handler)
  .parse();
