import {Link} from "react-router";
import {CpuIcon, HomeIcon, PaintbrushIcon, QrCodeIcon, SpoolIcon, UserRoundIcon} from "lucide-react";

export default function Navbar() {
    return (
        <nav className={"p-2 w-max grid grid-cols-6 gap-6 *:px-4 py-2 rounded-xl fixed top-4 left-1/2 -translate-x-1/2 border bg-background border-zinc-300 dark:border-zinc-900 **:text-foreground drop-shadow-2xl drop-shadow-neutral-700/10 z-10"}>
            <Link to={"/"}>
                <HomeIcon/>
                Home
            </Link>
            <Link to={"/filaments"}>
                <SpoolIcon/>
                Filaments
            </Link>
            <Link to={"/labels"}>
                <QrCodeIcon/>
                Labels
            </Link>
            <Link to={"/colors"}>
                <PaintbrushIcon/>
                Colors
            </Link>
            <Link to={"/device"}>
                <CpuIcon/>
                Device
            </Link>
            <Link to={"/account"}>
                <UserRoundIcon/>
                Account
            </Link>
        </nav>
    )
}