// package: pb
// file: pb.proto

import * as jspb from "google-protobuf";

export class GetBalanceRequest extends jspb.Message {
  getAddress(): Uint8Array | string;
  getAddress_asU8(): Uint8Array;
  getAddress_asB64(): string;
  setAddress(value: Uint8Array | string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetBalanceRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetBalanceRequest): GetBalanceRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetBalanceRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetBalanceRequest;
  static deserializeBinaryFromReader(message: GetBalanceRequest, reader: jspb.BinaryReader): GetBalanceRequest;
}

export namespace GetBalanceRequest {
  export type AsObject = {
    address: Uint8Array | string,
  }
}

export class GetBalanceResponse extends jspb.Message {
  getBalance(): number;
  setBalance(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetBalanceResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetBalanceResponse): GetBalanceResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetBalanceResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetBalanceResponse;
  static deserializeBinaryFromReader(message: GetBalanceResponse, reader: jspb.BinaryReader): GetBalanceResponse;
}

export namespace GetBalanceResponse {
  export type AsObject = {
    balance: number,
  }
}

export class GetHistoryRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetHistoryRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetHistoryRequest): GetHistoryRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetHistoryRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetHistoryRequest;
  static deserializeBinaryFromReader(message: GetHistoryRequest, reader: jspb.BinaryReader): GetHistoryRequest;
}

export namespace GetHistoryRequest {
  export type AsObject = {
  }
}

export class TransactionHistory extends jspb.Message {
  clearTransactionsList(): void;
  getTransactionsList(): Array<Transaction>;
  setTransactionsList(value: Array<Transaction>): void;
  addTransactions(value?: Transaction, index?: number): Transaction;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TransactionHistory.AsObject;
  static toObject(includeInstance: boolean, msg: TransactionHistory): TransactionHistory.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: TransactionHistory, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TransactionHistory;
  static deserializeBinaryFromReader(message: TransactionHistory, reader: jspb.BinaryReader): TransactionHistory;
}

export namespace TransactionHistory {
  export type AsObject = {
    transactionsList: Array<Transaction.AsObject>,
  }
}

export class Transaction extends jspb.Message {
  getBlockheight(): number;
  setBlockheight(value: number): void;

  getGas(): number;
  setGas(value: number): void;

  getMsg(): Uint8Array | string;
  getMsg_asU8(): Uint8Array;
  getMsg_asB64(): string;
  setMsg(value: Uint8Array | string): void;

  getPayer(): Uint8Array | string;
  getPayer_asU8(): Uint8Array;
  getPayer_asB64(): string;
  setPayer(value: Uint8Array | string): void;

  getSignature(): Uint8Array | string;
  getSignature_asU8(): Uint8Array;
  getSignature_asB64(): string;
  setSignature(value: Uint8Array | string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Transaction.AsObject;
  static toObject(includeInstance: boolean, msg: Transaction): Transaction.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Transaction, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Transaction;
  static deserializeBinaryFromReader(message: Transaction, reader: jspb.BinaryReader): Transaction;
}

export namespace Transaction {
  export type AsObject = {
    blockheight: number,
    gas: number,
    msg: Uint8Array | string,
    payer: Uint8Array | string,
    signature: Uint8Array | string,
  }
}

export class TransactionState extends jspb.Message {
  getState(): StateMap[keyof StateMap];
  setState(value: StateMap[keyof StateMap]): void;

  hasTx(): boolean;
  clearTx(): void;
  getTx(): Transaction | undefined;
  setTx(value?: Transaction): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TransactionState.AsObject;
  static toObject(includeInstance: boolean, msg: TransactionState): TransactionState.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: TransactionState, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TransactionState;
  static deserializeBinaryFromReader(message: TransactionState, reader: jspb.BinaryReader): TransactionState;
}

export namespace TransactionState {
  export type AsObject = {
    state: StateMap[keyof StateMap],
    tx?: Transaction.AsObject,
  }
}

export class SendBlockRequest extends jspb.Message {
  hasBlock(): boolean;
  clearBlock(): void;
  getBlock(): Block | undefined;
  setBlock(value?: Block): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SendBlockRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SendBlockRequest): SendBlockRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SendBlockRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SendBlockRequest;
  static deserializeBinaryFromReader(message: SendBlockRequest, reader: jspb.BinaryReader): SendBlockRequest;
}

export namespace SendBlockRequest {
  export type AsObject = {
    block?: Block.AsObject,
  }
}

export class SendTransactionRequest extends jspb.Message {
  hasTx(): boolean;
  clearTx(): void;
  getTx(): Transaction | undefined;
  setTx(value?: Transaction): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SendTransactionRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SendTransactionRequest): SendTransactionRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SendTransactionRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SendTransactionRequest;
  static deserializeBinaryFromReader(message: SendTransactionRequest, reader: jspb.BinaryReader): SendTransactionRequest;
}

export namespace SendTransactionRequest {
  export type AsObject = {
    tx?: Transaction.AsObject,
  }
}

export class GenericResponse extends jspb.Message {
  getMessage(): string;
  setMessage(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GenericResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GenericResponse): GenericResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GenericResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GenericResponse;
  static deserializeBinaryFromReader(message: GenericResponse, reader: jspb.BinaryReader): GenericResponse;
}

export namespace GenericResponse {
  export type AsObject = {
    message: string,
  }
}

export class Block extends jspb.Message {
  getIndex(): number;
  setIndex(value: number): void;

  getBlockhash(): Uint8Array | string;
  getBlockhash_asU8(): Uint8Array;
  getBlockhash_asB64(): string;
  setBlockhash(value: Uint8Array | string): void;

  clearTransactionsList(): void;
  getTransactionsList(): Array<Transaction>;
  setTransactionsList(value: Array<Transaction>): void;
  addTransactions(value?: Transaction, index?: number): Transaction;

  getPreviousBlockhash(): Uint8Array | string;
  getPreviousBlockhash_asU8(): Uint8Array;
  getPreviousBlockhash_asB64(): string;
  setPreviousBlockhash(value: Uint8Array | string): void;

  getNonce(): number;
  setNonce(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Block.AsObject;
  static toObject(includeInstance: boolean, msg: Block): Block.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Block, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Block;
  static deserializeBinaryFromReader(message: Block, reader: jspb.BinaryReader): Block;
}

export namespace Block {
  export type AsObject = {
    index: number,
    blockhash: Uint8Array | string,
    transactionsList: Array<Transaction.AsObject>,
    previousBlockhash: Uint8Array | string,
    nonce: number,
  }
}

export interface StateMap {
  SUCCESS: 0;
  FAIL: 1;
  PENDING: 2;
}

export const State: StateMap;

