import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import timezone from "dayjs/plugin/timezone";
import utc from "dayjs/plugin/utc";

dayjs.extend(relativeTime);
dayjs.extend(timezone);
dayjs.extend(utc);

export default function prettyDate(date) {
    return dayjs.tz(date, "UTC").fromNow();
}
