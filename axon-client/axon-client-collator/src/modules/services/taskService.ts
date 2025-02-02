import JSONbig from 'json-bigint'
import {CronJob} from "cron";
import {modules} from "../../container";
import {inject, injectable, LazyServiceIdentifer} from "inversify";
import {logger} from "axon-client-common/src/utils/logger";
import ScanService from "./scanService";
import {SidechainState} from "axon-client-common/src/modules/models/cells/sidechain_state";
import {CollatorPublishTaskTransformation} from "axon-client-common/src/modules/models/transformation/collator_publish_task";
import EngineService from "./engineService";
import {Task} from "axon-client-common/src/modules/models/cells/task";
import {CollatorSubmitChallengeTransformation} from "axon-client-common/src/modules/models/transformation/collator_submit_challenge";
import {CollatorSubmitTaskTransformation} from "axon-client-common/src/modules/models/transformation/collator_submit_task";

// @ts-ignore
const jsonbig = JSONbig({ useNativeBigInt: true,alwaysParseAsBig:true})


@injectable()
export default class TaskService {
    readonly #scanService: ScanService
    readonly #engineService: EngineService

    readonly #schedule = '*/5 * * * * *'

    #cronLock: boolean = false

    #info = (msg: string) => {
        logger.info(`TaskService: ${msg}`)
    }
    #error = (msg: string) => {
        logger.error(`TaskService: ${msg}`)
    }

    constructor(
        @inject(new LazyServiceIdentifer(() => modules[ScanService.name])) scanService: ScanService,
        @inject(new LazyServiceIdentifer(() => modules[EngineService.name])) engineService: EngineService,
    ) {
        this.#scanService = scanService
        this.#engineService = engineService
    }

    start = async () => {
        // public task
        new CronJob(this.#schedule, this.wrapperedTask, null, true)
        // submit task or challenge
        // refresh task
        //
    }

    readonly wrapperedTask = async () => {
        if (!this.#cronLock) {
            this.#cronLock = true
            try {
                this.#info('task job starts: ' + new Date())
                await this.task()
                this.#info('task job finishes: ' + new Date())

            } catch (e) {
                this.#error('task job error: ' + e)
            } finally {
                this.#cronLock = false
            }
        }
    }


    task = async () =>{
        // collator : publish -> [refresh] -> submit task/challenge -> publish

        //scan the sidechain status to see which status is now,
        //if it is waiting for submit, do another refresh check
        const code = await this.#scanService.scanCode()
        const state = await this.#scanService.scanSidechainState()
        const globalConfig = await this.#scanService.scanGlobalConfig()
        const config = await this.#scanService.scanSidechainConfig()

        if(state.status == SidechainState.STATUS_WAITING_FOR_PUBLISH){
            const bond = await this.#scanService.scanSidechainBond()

            const xfer = new CollatorPublishTaskTransformation(
                globalConfig,
                config,
                code,
                state,
                bond,
            )

            await this.#engineService.collatorPublishTask(xfer)

        }else if (state.status == SidechainState.STATUS_WAITING_FOR_SUBMIT){

            //if the interval is timed out, do refresh
            //if()

            const tasks = await this.#scanService.scanTask()

            if(tasks.some(task => task.mode === Task.CHALLENGE )){
                const fee = await this.#scanService.scanFee()
                const checkerInfos = await this.#scanService.scanCheckerInfo();
                const xfer = new CollatorSubmitChallengeTransformation(
                    globalConfig,
                    code,
                    config,
                    state,
                    fee,
                    checkerInfos
                )

                await this.#engineService.collatorSubmitChallenge(xfer)
            }else{
                const fee = await this.#scanService.scanFee()
                const checkerInfos = await this.#scanService.scanCheckerInfo();
                const xfer = new CollatorSubmitTaskTransformation(
                    globalConfig,
                    config,
                    code,
                    state,
                    fee,
                    checkerInfos
                )
                await this.#engineService.collatorSubmitTask(xfer)
            }

        }else{
            throw new Error("~")
        }
    }



}
