/**
* Structure of a user
*/
export interface User {
    username: string;
    vanity: string;
    avatar?: string | null;
    bio?: string | null;
    followers?: number | null;
    following?: string | null;
    verified?: boolean | null;
    deleted?: boolean | null;
    flags?: number | null;
}