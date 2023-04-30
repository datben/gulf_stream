import Image from "next/image";
import mypic from "../assets/seashell.png";

export const SeaShell = (w: number, h: number) => {
  return <Image src={mypic} alt="Picture of the author" width={w} height={h} />;
};
