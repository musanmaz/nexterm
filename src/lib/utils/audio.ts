import { Howl } from 'howler';

interface SoundMap {
  [key: string]: Howl;
}

class AudioManager {
  private sounds: SoundMap = {};
  private enabled = true;
  private volume = 0.5;

  init() {
    const soundFiles: Record<string, string> = {
      keystroke: '/sounds/keystroke.mp3',
      enter: '/sounds/enter.mp3',
      error: '/sounds/error.mp3',
      notification: '/sounds/notification.mp3',
      boot: '/sounds/boot.mp3',
      tab: '/sounds/tab.mp3',
    };

    for (const [name, src] of Object.entries(soundFiles)) {
      this.sounds[name] = new Howl({
        src: [src],
        volume: this.volume,
        preload: true,
      });
    }
  }

  play(name: string) {
    if (!this.enabled) return;
    this.sounds[name]?.play();
  }

  setVolume(vol: number) {
    this.volume = vol;
    Object.values(this.sounds).forEach((s) => s.volume(vol));
  }

  setEnabled(enabled: boolean) {
    this.enabled = enabled;
  }
}

export const audio = new AudioManager();
