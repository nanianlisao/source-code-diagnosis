/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface Compat {
  name: string
  mdnUrl: string
  description: string
  tags: Array<string>
  support: Support
}
export interface Status {
  experimental: boolean
  standardTrack: boolean
  deprecated: boolean
}
export interface Support {
  chrome: string
  chromeAndroid: string
  firefox: string
  firefoxAndroid: string
  safari: string
  safariIos: string
  edge: string
  node: string
  deno: string
}
export interface CompatBox {
  name: string
  span: Span
  compat: Compat
  filePath: string
  loc: Location
}
export interface Span {
  start: number
  end: number
}
export const enum CompatType {
  ClassConstructor = 0,
  ClassExtends = 1,
  ClassPrivateClassFieldsIn = 2,
  ClassPrivateClassFields = 3,
  ClassPrivateMethods = 4,
  ClassPublicClassFields = 5,
  ClassStaticClassFields = 6,
  ClassStaticInitializationBlocks = 7,
  ClassStatic = 8,
  Function = 9,
  Grammar = 10,
  Operator = 11,
  Statement = 12
}
export declare function checkBrowserSupported(target: string, options?: Options | undefined | null): Array<CompatBox>
export interface DangerStringLocation {
  rawValue: string
  matchDangerString: string
  start: number
  end: number
  filePath: string
}
export declare function getDangerStringsUsage(dangerStrings: Array<string>, options?: Options | undefined | null): Array<DangerStringLocation>
export interface ModuleMemberUsageLocation {
  libName: string
  memberName: string
  start: number
  end: number
  filePath: string
  loc: Location
}
export declare function getModuleMemberUsage(npmNameVec: Array<string>, options?: Options | undefined | null): Array<ModuleMemberUsageLocation>
export interface Target {
  chrome: string
}
export interface Options {
  pattern?: string
  ignore?: Array<string>
  cwd?: string
  concurrency?: number
}
export interface Location {
  start: Position
  end: Position
}
export interface Position {
  line: number
  col: number
}
