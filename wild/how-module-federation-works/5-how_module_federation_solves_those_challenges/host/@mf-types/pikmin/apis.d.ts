
    export type RemoteKeys = 'pikmin';
    type PackageType<T> = T extends 'pikmin' ? typeof import('pikmin') :any;