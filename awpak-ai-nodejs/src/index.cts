// This module is the CJS entry point for the library.

// The Rust addon.
import * as addon from './load.cjs';

// Use this declaration to assign types to the addon's exports,
// which otherwise by default are `any`.
declare module "./load.cjs" {
  function run_graph_once( path : string, input? : string ): Promise<string>;
  function run_graph( id : string, path : string, input? : string ): Promise<string>;
}


export async function run_graph_once( path : string, input? : string ) : Promise<string>
{
  return addon.run_graph_once( path, input );
}

export async function run_graph( id : string, path : string, input? : string ) : Promise<string>
{
  return addon.run_graph( id, path, input );
}