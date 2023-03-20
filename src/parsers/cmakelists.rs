use nom::{
    branch::{alt},
    bytes::complete::{tag, tag_no_case, take_until},
    character::complete::{
        alpha1, alphanumeric1, multispace0, not_line_ending,
    },
    combinator::{recognize},
    multi::{many0, many0_count},
    sequence::{delimited, pair},
    IResult,
};

fn comment_parser(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("#")(input)?;
    let (input, _) = not_line_ending(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, ()))
}

fn comment_parser0(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(comment_parser)(input)?;
    Ok((input, ()))
}

pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

fn project_name_parser(input: &str) -> nom::IResult<&str, &str> {
    let (input, _) = tag_no_case("project")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = comment_parser0(input)?;
    let (input, _) = delimited(multispace0, tag("("), multispace0)(input)?;

    let (input, _) = comment_parser0(input)?;
    let (input, project_name) = delimited(multispace0, identifier, multispace0)(input)?;

    let (input, _) = comment_parser0(input)?;
    let (input, _) = delimited(multispace0, tag(")"), multispace0)(input)?;
    Ok((input, project_name))
}

fn internal_cmakelists_name_parser(input: &str) -> IResult<&str, &str> {
    let (input, _) = take_until("project")(input)?;
    project_name_parser(input)
}

pub fn cmakelists_name_parser(input: &str) -> Option<String> {
    let (_, name) = internal_cmakelists_name_parser(input).ok()?;

    Some(name.to_string())
}

#[cfg(test)]
mod tests {
    use crate::parsers::cmakelists::{
        comment_parser, internal_cmakelists_name_parser, project_name_parser,
    };

    #[test]
    fn test_cmakelists() {
        let cmakelists = "cmake_minimum_required(VERSION 3.10.0)

list(APPEND CMAKE_MODULE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/ouster-sdk/cmake)
include(DefaultBuildType)

# ==== Project Name ====
project(ouster_ros)

# ==== Requirements ====
find_package(Eigen3 REQUIRED)
find_package(PCL REQUIRED COMPONENTS common)
find_package(tf2_eigen REQUIRED)
find_package(CURL REQUIRED)
find_package(Boost REQUIRED)

find_package(
  catkin REQUIRED
  COMPONENTS message_generation
             std_msgs
             sensor_msgs
             geometry_msgs
             pcl_conversions
             roscpp
             tf2
             tf2_ros
             tf2_eigen
             nodelet)

# ==== Options ====
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
add_compile_options(-Wall -Wextra)
option(CMAKE_POSITION_INDEPENDENT_CODE \"Build position independent code.\" ON)

# ==== Catkin ====
add_message_files(FILES PacketMsg.msg)
add_service_files(FILES GetConfig.srv SetConfig.srv GetMetadata.srv)
generate_messages(DEPENDENCIES std_msgs sensor_msgs geometry_msgs)

set(_ouster_ros_INCLUDE_DIRS
  \"include;ouster-sdk/ouster_client/include;ouster-sdk/ouster_client/include/optional-lite\")

catkin_package(
  INCLUDE_DIRS
    ${_ouster_ros_INCLUDE_DIRS}
  LIBRARIES
    ouster_ros
  CATKIN_DEPENDS
    roscpp
    message_runtime
    std_msgs
    sensor_msgs
    geometry_msgs
  DEPENDS
    EIGEN3
)

# ==== Libraries ====
# Build static libraries and bundle them into ouster_ros using the `--whole-archive` flag. This is
# necessary because catkin doesn't interoperate easily with target-based cmake builds. Object
# libraries are the recommended way to do this, but require >=3.13 to propagate usage requirements.
set(_SAVE_BUILD_SHARED_LIBS ${BUILD_SHARED_LIBS})
set(BUILD_SHARED_LIBS OFF)

option(BUILD_VIZ \"Enabled for Python build\" OFF)
option(BUILD_PCAP \"Enabled for Python build\" OFF)
find_package(OusterSDK REQUIRED)

set(BUILD_SHARED_LIBS ${_SAVE_BUILD_SHARED_LIBS})

# catkin adds all include dirs to a single variable, don't try to use targets
include_directories(${_ouster_ros_INCLUDE_DIRS} ${catkin_INCLUDE_DIRS})

# use only MPL-licensed parts of eigen
add_definitions(-DEIGEN_MPL2_ONLY)

add_library(ouster_ros src/os_ros.cpp)
target_link_libraries(ouster_ros PUBLIC ${catkin_LIBRARIES} ouster_build pcl_common PRIVATE
  -Wl,--whole-archive ouster_client -Wl,--no-whole-archive)
add_dependencies(ouster_ros ${PROJECT_NAME}_gencpp)

# ==== Executables ====
add_library(nodelets_os
  src/os_client_base_nodelet.cpp
  src/os_sensor_nodelet.cpp
  src/os_replay_nodelet.cpp
  src/os_cloud_nodelet.cpp
  src/os_image_nodelet.cpp)
target_link_libraries(nodelets_os ouster_ros ${catkin_LIBRARIES})
add_dependencies(nodelets_os ${PROJECT_NAME}_gencpp)

# ==== Install ====
install(
  TARGETS
    ouster_ros
    nodelets_os
  ARCHIVE DESTINATION ${CATKIN_PACKAGE_LIB_DESTINATION}
  LIBRARY DESTINATION ${CATKIN_PACKAGE_LIB_DESTINATION}
  RUNTIME DESTINATION ${CATKIN_PACKAGE_BIN_DESTINATION}
)

install(
  DIRECTORY ${_ouster_ros_INCLUDE_DIRS}
  DESTINATION ${CATKIN_PACKAGE_INCLUDE_DESTINATION}
)

install(
  FILES
    LICENSE
    nodelets_os.xml
  DESTINATION ${CATKIN_PACKAGE_SHARE_DESTINATION}
)

install(
  DIRECTORY
    launch
    config
  DESTINATION ${CATKIN_PACKAGE_SHARE_DESTINATION}
)
";
        if let Ok((_, project_name)) = internal_cmakelists_name_parser(cmakelists) {
            assert_eq!(project_name, "ouster_ros");
        } else {
            assert_eq!(1, 2);
        }
    }

    #[test]
    fn test_project_name() {
        let cmake_example = "project(example_name)";
        let parsed = project_name_parser(cmake_example);
        assert_eq!(parsed, Ok(("", "example_name")));
        let cmake_morespaced = "project   ( 
            example_ADS3Cname )";
        let parsed = project_name_parser(cmake_morespaced);
        assert_eq!(parsed, Ok(("", "example_ADS3Cname")));
        let cmake_spaced_and_commented = "project (
            # this is an important comment
            # before the project 
            # name
            example_name 
            )";
        let parsed = project_name_parser(cmake_spaced_and_commented);
        assert_eq!(parsed, Ok(("", "example_name")));
    }

    #[test]
    fn test_comment() {
        let comment = "#something";
        assert_eq!(comment_parser(comment), Ok(("", ())));

        let comment = "# this is a comment

            this is the rest";
        assert_eq!(comment_parser(comment), Ok(("this is the rest", ())));
        // assert_eq!(pinline_comment(comment), Ok(()))
    }
}
