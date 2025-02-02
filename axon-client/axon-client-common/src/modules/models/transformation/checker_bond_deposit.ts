import {Transformation} from './interfaces/transformation'

/*
CheckerBondDeposit

Muse Token Cell             ->          Check Bond Cell

No way to monitor this pattern, regard all check bond cell trustless

 */

export class CheckerBondDepositTransformation implements Transformation {

    processed: boolean = false;
    skip: boolean = false;
    composedTx?: CKBComponents.RawTransaction = undefined
    composedTxHash?: string = undefined

    constructor() {
    }

    toCellDeps(): Array<CKBComponents.CellDep> {
        return [];
    }

    toCellInput(): Array<CKBComponents.CellInput> {
        return []
    }

    toCellOutput(): Array<CKBComponents.CellOutput> {
        return []
    }

    toCellOutputData(): Array<string> {

        return []
    }

    toWitness(): Array<CKBComponents.WitnessArgs> {
        return [];
    }


}
