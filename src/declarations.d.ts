declare module 'howler' {
  export class Howl {
    constructor(options: Record<string, unknown>);
    play(): void;
    volume(vol?: number): number;
  }
}
