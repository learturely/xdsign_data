use cxlib_types::{Geoaddr, LocationPreprocessorTrait};
use static_read_only_table::{table, ReadOnlyTable};

#[derive(Debug, Clone)]
pub struct GeographicPoint {
    lat: f64,
    lon: f64,
    addr: &'static str,
}

impl GeographicPoint {
    /// 创建新点并进行有效性检查
    pub fn new(lat: f64, lon: f64, addr: &'static str) -> Result<Self, &'static str> {
        if !(-90.0..=90.0).contains(&lat) {
            return Err("Latitude must be between -90 and 90 degrees");
        }
        if !(-180.0..=180.0).contains(&lon) {
            return Err("Longitude must be between -180 and 180 degrees");
        }
        Ok(Self { lat, lon, addr })
    }
    /// 使用 Haversine 公式计算两点间的大圆距离（单位：公里）
    pub fn haversine_distance_meters_to(self: &GeographicPoint, point2: &GeographicPoint) -> f64 {
        const EARTH_RADIUS_M: f64 = 6371393.0;
        // 将角度转换为弧度
        let lat1 = self.lat.to_radians();
        let lon1 = self.lon.to_radians();
        let lat2 = point2.lat.to_radians();
        let lon2 = point2.lon.to_radians();

        // 计算纬度差和经度差
        let delta_lat = lat2 - lat1;
        let delta_lon = lon2 - lon1;

        // Haversine 公式核心计算
        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        // 最终距离（米）
        EARTH_RADIUS_M * c
    }
}
/// See [here](https://github.com/Pairman/Xdcheckin/blob/main/src/xdcheckin/server/static/g_locations.js).
pub static LOCATIONS: ReadOnlyTable<&str, GeographicPoint, 30> = table! {
    "A楼": GeographicPoint {
        lat: 34.133171,
        lon: 108.837420,
        addr: "西安市长安区兴隆街道内环北路西安电子科技大学(南校区)"
    },
    "B楼": GeographicPoint{
        lat: 34.132297,
        lon: 108.838367,
        addr: "西安市长安区兴隆街道内环北路西安电子科技大学(南校区)"
    },
    "C楼": GeographicPoint{
        lat: 34.131125,
        lon: 108.838983,
        addr: "西安市长安区兴隆街道内环北路西安电子科技大学(南校区)"
    },
    "D楼": GeographicPoint{
        lat: 34.130856,
        lon: 108.841579,
        addr: "西安市长安区兴隆街道内环北路西安电子科技大学(南校区)"
    },
    "EI楼": GeographicPoint{
        lat: 34.130878,
        lon: 108.839863,
        addr: "西安市长安区兴隆街道内环北路西安电子科技大学(南校区)"
    },
    "EII楼": GeographicPoint{
        lat: 34.130856,
        lon: 108.841579,
        addr: "西安市长安区兴隆街道内环北路西安电子科技大学(南校区)"
    },
    "EIII楼":GeographicPoint {
        lat: 34.130056,
        lon: 108.843268,
        addr: "西安市长安区兴隆街道内环北路西安电子科技大学(南校区)"
    },
    "F楼": GeographicPoint{
        lat: 34.130654,
        lon: 108.843016,
        addr: "西安市长安区兴隆街道内环北路西安电子科技大学(南校区)"
    },
    "G楼": GeographicPoint{
        lat: 34.129660,
        lon: 108.845244,
        addr: "西安市长安区兴隆街道内环北路西安电子科技大学(南校区)"
    },
    "信远楼": GeographicPoint{
        lat: 34.131640,
        lon: 108.845415,
        addr: "西安市长安区兴隆街道外环北路西安电子科技大学(南校区)"
    },
    "图书馆": GeographicPoint{
        lat: 34.131125,
        lon: 108.838983,
        addr: "西安市长安区兴隆街道内环南路西安电子科技大学(南校区)"
    },
    "大学生活动中心": GeographicPoint{
        lat: 34.134972,
        lon: 108.835282,
        addr: "西安市长安区兴隆街道梧桐大道西安电子科技大学(南校区)"
    },
    "北操场": GeographicPoint{
        lat: 34.137362,
        lon: 108.837906,
        addr: "西安市长安区兴隆街道梧桐大道西安电子科技大学(南校区)"
    },
    "北篮球场": GeographicPoint{
        lat: 34.134972,
        lon: 108.835282,
        addr: "西安市长安区兴隆街道梧桐大道西安电子科技大学(南校区)"
    },
    "北乒乓球场": GeographicPoint{
        lat: 34.134972,
        lon: 108.835282,
        addr: "西安市长安区兴隆街道梧桐大道西安电子科技大学(南校区)"
    },
    "游泳中心": GeographicPoint{
        lat: 34.134972,
        lon: 108.835282,
        addr: "西安市长安区兴隆街道梧桐大道西安电子科技大学(南校区)"
    },
    "南操场": GeographicPoint{
        lat: 34.132559,
        lon: 108.832542,
        addr: "西安市长安区兴隆街道梧桐大道西安电子科技大学(南校区)"
    },
    "南篮球场": GeographicPoint{
        lat: 34.128472,
        lon: 108.832443,
        addr: "西安市长安区兴隆街道梧桐大道西安电子科技大学(南校区)"
    },
    "南乒乓球场": GeographicPoint{
        lat: 34.129376,
        lon: 108.834375,
        addr: "西安市长安区兴隆街道梧桐大道西安电子科技大学(南校区)"
    },
    "远望谷体育馆": GeographicPoint{
        lat: 34.126418,
        lon: 108.844544,
        addr: "西安市长安区兴隆街道内环南路西安电子科技大学(南校区)"
    },
    "博物馆": GeographicPoint{
        lat: 34.125589,
        lon: 108.844337,
        addr: "西安市长安区兴隆街道内环南路西安电子科技大学(南校区)"
    },
    "网安大楼": GeographicPoint{
        lat: 34.128353,
        lon: 108.842010,
        addr: "西安市长安区兴隆街道内环南路西安电子科技大学(南校区)"
    },
    "礼仪广场": GeographicPoint{
        lat: 34.132006,
        lon: 108.842118,
        addr: "西安市长安区兴隆街道内环北路西安电子科技大学(南校区)"
    },
    "西大楼": GeographicPoint{
        lat: 34.240976,
        lon: 108.923468,
        addr: "西安市雁塔区电子城街道电子大道北段西安电子科技大学"
    },
    "阶梯楼": GeographicPoint{
        lat: 34.238268,
        lon: 108.926841,
        addr: "西安市雁塔区电子城街道电子大道北段西安电子科技大学"
    },
    "北校区体育馆": GeographicPoint{
        lat: 34.232305,
        lon: 108.915958,
        addr: "西安市雁塔区电子城街道电子大道北段西安电子科技大学"
    },
    "北校区室外篮球场": GeographicPoint{
        lat: 34.232459,
        lon: 108.917250,
        addr: "西安市雁塔区电子城街道电子大道北段西安电子科技大学"
    },
    "北校区操场": GeographicPoint{
        lat: 34.231810,
        lon: 108.918339,
        addr: "西安市雁塔区电子城街道电子大道北段西安电子科技大学"
    },
    "北校区图书馆": GeographicPoint{
        lat: 34.231394,
        lon: 108.916595,
        addr: "西安市雁塔区电子城街道电子大道北段西安电子科技大学"
    },
    "北校区会议中心": GeographicPoint{
        lat: 34.230493,
        lon: 108.917517,
        addr: "西安市雁塔区电子城街道电子大道北段西安电子科技大学"
    }
};

pub struct LocationPreprocessor;
impl LocationPreprocessorTrait for LocationPreprocessor {
    fn do_preprocess(&self, geoaddr: Geoaddr) -> Geoaddr {
        let location = geoaddr.location();
        let geolocation = GeographicPoint::new(
            location.lat.parse().expect("地理坐标不合法。"),
            location.lon.parse().expect("地理坐标不合法。"),
            "doesntmatter",
        )
        .expect("地理坐标不合法。");
        let max_offset = 200.0;
        let max_distance = f64::MAX;
        let mut geoaddr = geoaddr;
        for v in LOCATIONS.values() {
            let distance = v.haversine_distance_meters_to(&geolocation);
            if distance < max_offset && distance < max_distance {
                *geoaddr.get_addr_mut() = v.addr.to_string();
            }
        }
        #[cfg(debug_assertions)]
        if geoaddr.addr().contains("菜鸟驿站") {
            *geoaddr.get_addr_mut() = "TEST".to_string();
        }
        geoaddr
    }
}

#[cfg(test)]
mod tests {
    use crate::LOCATIONS;

    #[test]
    fn print_addrs() {
        println!("{:?}", LOCATIONS.keys());
    }
    #[test]
    fn print_locations() {
        println!("{:?}", LOCATIONS.values());
    }
}

#[cfg(test)]
mod geographic_point_tests {
    use crate::{GeographicPoint, LOCATIONS};

    // 基准测试：纽约到洛杉矶
    const NY_LAT: f64 = 40.7128;
    const NY_LON: f64 = -74.0060;
    const LA_LAT: f64 = 34.0522;
    const LA_LON: f64 = -118.2437;
    const EXPECTED_DISTANCE: f64 = 3944000.0; // ±50 km 可接受范围

    #[test]
    fn test_known_distance() {
        let ny = GeographicPoint::new(NY_LAT, NY_LON, "纽约").unwrap();
        let la = GeographicPoint::new(LA_LAT, LA_LON, "洛杉矶").unwrap();

        let distance = ny.haversine_distance_meters_to(&la);
        println!("{distance}");
        assert!((distance - EXPECTED_DISTANCE).abs() <= 50000.0);
    }

    #[test]
    fn test_same_point() {
        let point = GeographicPoint::new(35.0, 139.0, "?").unwrap();
        assert_eq!(point.haversine_distance_meters_to(&point), 0.0);
    }

    #[test]
    fn test_invalid_latitude() {
        assert!(GeographicPoint::new(100.0, 0.0, "?").is_err());
        assert!(GeographicPoint::new(-100.0, 0.0, "?").is_err());
    }

    #[test]
    fn test_invalid_longitude() {
        assert!(GeographicPoint::new(0.0, 200.0, "?").is_err());
        assert!(GeographicPoint::new(0.0, -200.0, "?").is_err());
    }

    #[test]
    fn test_short_distance() {
        let points = &LOCATIONS.values()[..30 - 7];
        let optimized_distances: Vec<_> = points
            .iter()
            .enumerate()
            .flat_map(|(i, point1)| {
                points[(i + 1)..]
                    .iter()
                    .enumerate()
                    .map(move |(j, point2)| (i, i + 1 + j, point1, point2))
            })
            .map(|(i, j, point1, point2)| (i, j, point1.haversine_distance_meters_to(point2)))
            .collect();

        println!("\nOptimized results:");
        for (i, j, distance) in optimized_distances {
            assert!(distance < 1500.0);
            println!("Pair ({i}, {j}): {distance:.2} m");
        }
    }
}
