import {Link} from "react-router";
import GeneralNotFound from "./GeneralNotFound.tsx";

export default function PageNotFound() {
    return (
        <GeneralNotFound title={"Page Not Found"}>
            <p>This page was not found</p>
            <p>Check if the link is correct, or return {" "}
                <Link to={"/"}>Home</Link>
            </p>
        </GeneralNotFound>
    )
}