import { Buffer } from 'buffer';

export type TranspiledModule = {
  filename: string;
  code: string;
  map: string | null;
};

export type MinifiedModule = {
  code: string;
  map: string | null;
};

export const transpile: (filename: string, source: Buffer) => Promise<TranspiledModule>;
export const minify: (source: string) => Promise<MinifiedModule>;
