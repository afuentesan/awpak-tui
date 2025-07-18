import { DataFromVariant, DataOperationAdd, DataOperationLen, DataOperationSubstract, DataOperationVariant, FromAgentHistory, FromAgentHistoryContentFirst, FromAgentHistoryContentFirstMessage, FromAgentHistoryContentFull, FromAgentHistoryContentFullMessages, FromAgentHistoryContentItem, FromAgentHistoryContentItemMessage, FromAgentHistoryContentLast, FromAgentHistoryContentLastMessage, FromAgentHistoryContentRange, FromAgentHistoryContentRangeMessages, FromAgentHistoryContentVariant, FromConcat, FromContext, FromInput, FromNull, FromOperation, FromParsedInput, FromStatic, type DataFrom, type DataOperation, type FromAgentHistoryContent } from "../model/data";
import { DataComparatorAnd, DataComparatorEq, DataComparatorFalse, DataComparatorGt, DataComparatorLt, DataComparatorNot, DataComparatorNotEq, DataComparatorOr, DataComparatorRegex, DataComparatorTrue, DataComparatorVariant, type DataComparator } from "../model/data_comparator";
import { WebClientBodyForm, WebClientBodyJson, WebClientBodyVariant, type WebClientBody } from "../model/web_client";
import { is_type_in_enum } from "./form_utils";

export function is_empty( data : any ) : boolean
{
    return typeof( data ) === "undefined" || data === null;
}

export function not_empty_or_string_eq( data : any, key : string ) : boolean
{
    if( typeof( data ) === "undefined" || data === null ) return false;

    if( ! is_empty( data?.[ key ] ) ) return true;

    if( typeof( data ) === "string" && data == key ) return true;

    return false;
}

export function new_data_operation_variant( old : DataOperation, new_variant : string ) : DataOperation | undefined
{
    if( ! is_type_in_enum( DataOperationVariant, new_variant ) ) { return undefined; }

    new_variant = new_variant as DataOperationVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == DataOperationVariant.Len )
    {
        return new DataOperationLen();
    }
    else if( new_variant == DataOperationVariant.Add )
    {
        return new DataOperationAdd();
    }
    else if( new_variant == DataOperationVariant.Substract )
    {
        return new DataOperationSubstract();
    }

    return undefined;
}

export function new_data_comparator_variant( old : DataComparator, new_variant : string ) : DataComparator | undefined
{
    if( ! is_type_in_enum( DataComparatorVariant, new_variant ) ) { return undefined; }

    new_variant = new_variant as DataComparatorVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == DataComparatorVariant.Eq )
    {
        return new DataComparatorEq();
    }
    else if( new_variant == DataComparatorVariant.NotEq )
    {
        return new DataComparatorNotEq();
    }
    else if( new_variant == DataComparatorVariant.Gt )
    {
        return new DataComparatorGt();
    }
    else if( new_variant == DataComparatorVariant.Lt )
    {
        return new DataComparatorLt();
    }
    else if( new_variant == DataComparatorVariant.Regex )
    {
        return new DataComparatorRegex()
    }
    else if( new_variant == DataComparatorVariant.And )
    {
        return new DataComparatorAnd();
    }
    else if( new_variant == DataComparatorVariant.Or )
    {
        return new DataComparatorOr();
    }
    else if( new_variant == DataComparatorVariant.Not )
    {
        return new DataComparatorNot();
    }
    else if( new_variant == DataComparatorVariant.True )
    {
        return new DataComparatorTrue();
    }
    else if( new_variant == DataComparatorVariant.False )
    {
        return new DataComparatorFalse();
    }

    return undefined;
}

export function new_body_variant( old : WebClientBody, new_variant : string ) : WebClientBody | undefined
{
    if( ! is_type_in_enum( WebClientBodyVariant, new_variant ) ) { return undefined; }

    if( ! old ) old = new WebClientBodyJson();

    new_variant = new_variant as WebClientBodyVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == WebClientBodyVariant.Json )
    {
        return new WebClientBodyJson();
    }
    else if( new_variant == WebClientBodyVariant.Form )
    {
        return new WebClientBodyForm();
    }
}

export function new_data_from_agent_history_content( old : FromAgentHistoryContent, new_variant : string ) : FromAgentHistoryContent | undefined
{
    if( ! is_type_in_enum( FromAgentHistoryContentVariant, new_variant ) ) { return undefined; }

    new_variant = new_variant as FromAgentHistoryContentVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == FromAgentHistoryContentVariant.Full )
    {
        return new FromAgentHistoryContentFull();
    }
    else if( new_variant == FromAgentHistoryContentVariant.FullMessages )
    {
        return new FromAgentHistoryContentFullMessages();
    }
    else if( new_variant == FromAgentHistoryContentVariant.First )
    {
        return new FromAgentHistoryContentFirst();
    }
    else if( new_variant == FromAgentHistoryContentVariant.FirstMessage )
    {
        return new FromAgentHistoryContentFirstMessage();
    }
    else if( new_variant == FromAgentHistoryContentVariant.Last )
    {
        return new FromAgentHistoryContentLast();
    }
    else if( new_variant == FromAgentHistoryContentVariant.LastMessage )
    {
        return new FromAgentHistoryContentLastMessage();
    }
    else if( new_variant == FromAgentHistoryContentVariant.Range )
    {
        let ret = new FromAgentHistoryContentRange();

        if( old._variant == FromAgentHistoryContentVariant.RangeMessages )
        {
            ret.from = old.from;
            ret.to = old.to;
        }

        return ret;
    }
    else if( new_variant == FromAgentHistoryContentVariant.RangeMessages )
    {
        let ret = new FromAgentHistoryContentRangeMessages();

        if( old._variant == FromAgentHistoryContentVariant.Range )
        {
            ret.from = old.from;
            ret.to = old.to;
        }

        return ret;
    }
    else if( new_variant == FromAgentHistoryContentVariant.Item )
    {
        let ret = new FromAgentHistoryContentItem();

        if( old._variant == FromAgentHistoryContentVariant.ItemMessage )
        {
            ret.value = old.value;
        }

        return ret;
    }
    else if( new_variant == FromAgentHistoryContentVariant.ItemMessage )
    {
        let ret = new FromAgentHistoryContentItemMessage();

        if( old._variant == FromAgentHistoryContentVariant.Item )
        {
            ret.value = old.value;
        }

        return ret;
    }
}

export function new_data_from_variant( old : DataFrom, new_variant : string ) : DataFrom | undefined
{
    if( ! is_type_in_enum( DataFromVariant, new_variant ) ) { return undefined; }

    new_variant = new_variant as DataFromVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == DataFromVariant.Context )
    {
        return new FromContext();
    }
    else if( new_variant == DataFromVariant.Input )
    {
        return new FromInput();
    }
    else if( new_variant == DataFromVariant.ParsedInput )
    {
        return new FromParsedInput();
    }
    else if( new_variant == DataFromVariant.Static )
    {
        return new FromStatic();
    }
    else if( new_variant == DataFromVariant.Operation )
    {
        return new FromOperation();
    }
    else if( new_variant == DataFromVariant.Concat )
    {
        return new FromConcat();
    }
    else if( new_variant == DataFromVariant.Null )
    {
        return new FromNull();
    }
    else if( new_variant == DataFromVariant.AgentHistory )
    {
        return new FromAgentHistory();
    }
    else
    {
        return undefined;
    }
}