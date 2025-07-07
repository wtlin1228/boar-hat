
    export type RemoteKeys = 'kirby';
    type PackageType<T> = T extends 'kirby' ? typeof import('kirby') :any;