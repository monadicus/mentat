# SnarkOS Tracking Docs

This document will keep track of what is and isn't currently possible with the available snarkOS implementation, with regards to Rosetta API functionality.

## Table

Endpoints denoted with [I] are optional Rosetta Indexer endpoints, and are not mandatory to be implemented in order to conform to the Rosetta API spec.

|     Rosetta Endpoint     |       Possible?        |      SnarkOS Endpoints Involved      |
|:------------------------:|:----------------------:|:------------------------------------:|
|     /account/balance     |           No           |                 N/A                  |
|      /account/coins      |           No           |                 N/A                  |
|          /block          |          Yes           |               getblock               |
|    /block/transaction    |          Yes           | gettransaction, getblocktransactions |
|          /call           |           No           |                 N/A                  |
|  /construction/combine   |           No           |                 N/A                  |
|   /construction/derive   |           No           |                 N/A                  |
|    /construction/hash    |           No           |                 N/A                  |
|  /construction/metadata  |           No           |                 N/A                  |
|   /construction/parse    |           No           |                 N/A                  |
|  /construction/payloads  |           No           |                 N/A                  |
| /construction/preprocess |           No           |                 N/A                  |
|   /construction/submit   |          Yes           |           sendtransaction            |
|    /events/blocks [I]    |           No           |                 N/A                  |
|         /mempool         |          Yes           |            getmemorypool             |
|   /mempool/transaction   |           No           |    getmemorypool, gettransaction     |
|      /network/list       |           No           |                 N/A                  |
|     /network/options     |           No           |                 N/A                  |
|     /network/status      |           No           |                 N/A                  |
| /search/transactions [I] |           No           |                 N/A                  |
