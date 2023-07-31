// Imports
import 'module';
import './file';
import './file.mjs';
import 'styles.css';
import icon from '../icons/add.svg';
import image from './img/hero.png';

import { foo } from 'module';
import { bar } from '../file';
import * as ns1 from './file';

// Dynamic imports
await import('module');
await import('./nested/file');
await import('../parent/file');

// Exports
export * from './file';
export type * from './nested/types';
export {} from '../parent/file.mjs';
export { foo, bar } from './file';
export * as ns from './file/../weird/path';
