export default interface Base {
    type: string;
    optional?: boolean;
    unique?: boolean;
    /**
     * The chance that this field will be set if unique is set to true.
     *
     * @minimum 0
     * @maximum 1
     */
    chanceUnset?: number;
    transform?: string;
    transformArgs?: any;
}