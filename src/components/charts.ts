import { graphic } from "echarts";

interface DataEntry {
    learned: number;
    familiar: number;
    unfamiliar: number;
    new: number;
}

interface Statistic {
    [date: string]: DataEntry;
  }

export function generateGradientStackedAreaChartOptionFromStatistic(data:Statistic) {
    const colors = ['#80FFA5', '#00DDFF', '#37A2FF', '#FF0087', '#FFBF00'];
    const categories = Object.keys(data);
    const seriesData: {
        learned: number[];
        familiar: number[];
        unfamiliar: number[];
        new: number[];
      } = {
      learned: [],
      familiar: [],
      unfamiliar: [],
      new: []
    };

    categories.forEach(date => {
      seriesData.learned.push(data[date].learned);
      seriesData.familiar.push(data[date].familiar);
      seriesData.unfamiliar.push(data[date].unfamiliar);
      seriesData.new.push(data[date].new);
    });

    return {
      color: colors,
      title: {
        text: 'Learning statistics'
      },
      tooltip: {
        trigger: 'axis',
        axisPointer: {
          type: 'cross',
          label: {
            backgroundColor: '#6a7985'
          }
        }
      },
      legend: {
        data: ['learned', 'familiar', 'unfamiliar', 'new']
      },
      toolbox: {
        feature: {
          saveAsImage: {}
        }
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        containLabel: true
      },
      xAxis: [
        {
          type: 'category',
          boundaryGap: false,
          data: categories
        }
      ],
      yAxis: [
        {
          type: 'value'
        }
      ],
      series: [
        {
          name: 'learned',
          type: 'line',
          stack: 'Total',
          smooth: true,
          lineStyle: {
            width: 0
          },
          showSymbol: false,
          areaStyle: {
            opacity: 0.8,
            color: new graphic.LinearGradient(0, 0, 0, 1, [
              { offset: 0, color: 'rgb(128, 255, 165)' },
              { offset: 1, color: 'rgb(1, 191, 236)' }
            ])
          },
          emphasis: {
            focus: 'series'
          },
          data: seriesData.learned
        },
        {
          name: 'familiar',
          type: 'line',
          stack: 'Total',
          smooth: true,
          lineStyle: {
            width: 0
          },
          showSymbol: false,
          areaStyle: {
            opacity: 0.8,
            color: new graphic.LinearGradient(0, 0, 0, 1, [
              { offset: 0, color: 'rgb(0, 221, 255)' },
              { offset: 1, color: 'rgb(77, 119, 255)' }
            ])
          },
          emphasis: {
            focus: 'series'
          },
          data: seriesData.familiar
        },
        {
          name: 'unfamiliar',
          type: 'line',
          stack: 'Total',
          smooth: true,
          lineStyle: {
            width: 0
          },
          showSymbol: false,
          areaStyle: {
            opacity: 0.8,
            color: new graphic.LinearGradient(0, 0, 0, 1, [
              { offset: 0, color: 'rgb(55, 162, 255)' },
              { offset: 1, color: 'rgb(116, 21, 219)' }
            ])
          },
          emphasis: {
            focus: 'series'
          },
          data: seriesData.unfamiliar
        },
        {
          name: 'new',
          type: 'line',
          stack: 'Total',
          smooth: true,
          lineStyle: {
            width: 0
          },
          showSymbol: false,
          areaStyle: {
            opacity: 0.8,
            color: new graphic.LinearGradient(0, 0, 0, 1, [
              { offset: 0, color: 'rgb(255, 0, 135)' },
              { offset: 1, color: 'rgb(135, 0, 157)' }
            ])
          },
          emphasis: {
            focus: 'series'
          },
          data: seriesData.new
        }
      ]
    };
  }