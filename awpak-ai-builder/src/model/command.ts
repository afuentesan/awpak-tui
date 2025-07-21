import { FromStatic, type DataFrom } from "./data";

export class Command
{
    command : DataFrom = new FromStatic();
    args : Array<DataFrom> = [];
    output : Array<CommandOutput> = [];
    timeout : number | undefined;
}

export type CommandOutput = CommandOutputOut | CommandOutputErr | CommandOutputSuccess | CommandOutputCode | CommandOutputObject;

export enum CommandOutputVariant
{
    Out = "Out",
    Err = "Err",
    Success = "Success",
    Code = "Code",
    Object = "Object"
}

export class CommandOutputOut
{
    readonly _variant = CommandOutputVariant.Out;

    prefix : string | undefined;
    suffix : string | undefined;
}

export class CommandOutputErr
{
    readonly _variant = CommandOutputVariant.Err;

    prefix : string | undefined;
    suffix : string | undefined;
}

export class CommandOutputSuccess
{
    readonly _variant = CommandOutputVariant.Success;

    prefix : string | undefined;
    suffix : string | undefined;
}

export class CommandOutputCode
{
    readonly _variant = CommandOutputVariant.Code;

    prefix : string | undefined;
    suffix : string | undefined;
}

export class CommandOutputObject
{
    readonly _variant = CommandOutputVariant.Object;

    prefix : string | undefined;
    suffix : string | undefined;
}