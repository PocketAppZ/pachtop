import useServerEventsContext from "@/hooks/useServerEventsContext";
import StatsRing from "@/components/stats-ring";
import formatOverallStats from "@/features/metrics/utils/format-overall-stats";
import React from "react";

import { IconCpu2 } from "@tabler/icons-react";

const MemoryStatsRing: React.FC = ({}) => {
  const { memory } = useServerEventsContext();

  const available = memory?.at(-1)?.total || 0;
  const used = memory?.at(-1)?.used || 0;
  const progress = memory?.at(-1)?.usedPercentage || 0;

  const stats = React.useMemo(() => formatOverallStats(used, available), [used, available]);

  return <StatsRing color="cyan" Icon={IconCpu2} stats={stats} label="Memory" progress={progress} />;
};

export default MemoryStatsRing;