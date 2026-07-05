import {Typography} from "@heroui/react";
import {Link} from "react-router";
import {ExternalLinkIcon, GlobeIcon} from "lucide-react";
import {SiGithub, SiGmail, SiInstagram} from "@icons-pack/react-simple-icons";

export default function Footer() {
    const year = new Date().getFullYear()

    return (
        <footer className={"flex gap-48 bg-neutral-950 px-56 py-12 **:text-neutral-300 **:font-light rounded-t-4xl mx-4"}>
            <div>
                <Title text={"Filamentizator"} subtext={"v3"}/>
                <p className={"text-sm"}>© 2025 - {year}</p>
                <p>Peter Cyprich</p>
            </div>
            <div>
                <Title text={"Contact & Socials"}/>
                <div className={"line-through"}>
                    <ExternalLink
                        to={"https://www.cyprich.xyz"}
                        text={"Website | www.cyprich.xyz"}
                        prefixIcon={ <GlobeIcon className={"size-4"}/> }
                    />
                </div>
                <ExternalLink
                    to={"https://www.instagram.com/peter.tar.gz"}
                    text={"Instagram | peter.tar.gz"}
                    prefixIcon={ <SiInstagram className={"size-4"}/> }
                />
                <ExternalLink
                    to={"https://github.com/cyprich"}
                    text={"Github | cyprich"}
                    prefixIcon={ <SiGithub className={"size-4"}/> }
                />
                <ExternalLink
                    to={"mailto:cypooriginal@gmail.com"}
                    text={"Gmail | cypooriginal@gmail.com"}
                    prefixIcon={ <SiGmail className={"size-4"}/> }
                />

            </div>
            <div>
                <Title text={"Other Links"}/>
                <Link to={"/about"}>About</Link>
                <ExternalLink to={"https://github.com/cyprich/filamentizator"} text={"Source Code"}/>
                <p className={"clickable-no-scale line-through"}>Documentation & API reference</p>
            </div>
        </footer>
    )
}

type TitleProps = {
    text: string;
    subtext?: string;
}

function Title(props: TitleProps) {
    return (
        <Typography
            type={"h4"}
            className={"font-semibold! mb-1"}
        >
            {props.text}
            {
                props.subtext && <span className={"font-extralight pl-1.5"}>
                    {props.subtext}
                </span>
            }
        </Typography>

    )
}

type ExternalLinkProps = {
    to: string;
    text: string;
    prefixIcon?: any;
}

function ExternalLink(props: ExternalLinkProps) {
    return (
        <Link to={props.to} target={"_blank"} className={"clickable-no-scale flex items-center gap-1.5"}>
            { props.prefixIcon && props.prefixIcon}
            {props.text}
            <ExternalLinkIcon className={"size-4"} strokeWidth={1.5}/>
        </Link>
    )
}