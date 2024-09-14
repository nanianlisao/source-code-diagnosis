/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface AstNode {
  span: Span
  loc: Location
}
export interface Location {
  start: Position
  end: Position
}
export interface Position {
  line: number
  col: number
}
export interface Span {
  start: number
  end: number
}
export interface GlobOptions {
  pattern?: string
  ignore?: Array<string>
  cwd?: string
  concurrency?: number
}
export interface Response {
  rawValue: string
  matchDangerString: string
  filePath: string
  astNode: AstNode
}
export interface Response {
  libName: string
  memberName: string
  filePath: string
  astNode: AstNode
}
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
  compat: Compat
  filePath: string
  astNode: AstNode
}
export declare function checkDangerStrings(dangerStrings: Array<string>, options?: GlobOptions | undefined | null): Array<Response>
export declare function getModuleMemberUsage(npmNameVec: Array<string>, options?: GlobOptions | undefined | null): Array<Response>
export declare function checkBrowserSupported(target: string, options?: GlobOptions | undefined | null): Array<CompatBox>
export declare function checkBrowserSupportedWithSourceCode(target: string, sourceCode: string): Array<CompatBox>
