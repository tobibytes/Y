import {motion} from "motion/react"
import { Card, CardContent, CardFooter, CardHeader } from "../ui/card"
import { Heart, MessageCircle, RefreshCwIcon } from "lucide-react"
import { Button } from "../ui/button"
import Image from "next/image"
import { useRouter } from "next/navigation"
import { Avatar, AvatarFallback, AvatarImage } from "../ui/avatar"
export default function Post({name,userName,profilePic,postedAt, content, media, metrics}:{
    name: string,
    userName: string,
    profilePic: string,
    postedAt: string,
    content: string,
    media: string[], 
    metrics:object}
){
const router = useRouter()
const handleClick = () => {
    router.push("/post")
}
return(
    
    <motion.div
    whileHover={{scale: 1.1}}>
        <Card onClick={()=>{console.log("routing logic here")}}>
            <CardHeader className="flex gap-4">
                <Avatar>
                    <AvatarImage src={profilePic} />
                    <AvatarFallback>SC</AvatarFallback>
                </Avatar>
                <div>
                    <h1>{name}</h1>
                    <p>{userName}</p>
                </div>
                
                <p className="ml-auto">{postedAt}</p>
            </CardHeader>
            <CardContent>
                {content}
               
                {media?.map((medias:string, index:number) => (
                    <Image key={index} src={medias} alt={`Media ${index + 1}`} />
                ))}
            
            </CardContent>
            <CardFooter>
                <Button variant={'ghost'}>{metrics.comments}<MessageCircle/></Button>
                <Button variant={'ghost'}>{metrics.reposts}<RefreshCwIcon/></Button>
                <Button variant={'ghost'}>{metrics.likes}<Heart/></Button>
            </CardFooter>
        </Card>
    </motion.div>
)
}