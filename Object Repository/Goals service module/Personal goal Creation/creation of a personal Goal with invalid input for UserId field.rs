<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>creation of own goals service</description>
   <name>creation of a personal Goal with invalid input for UserId field</name>
   <tag></tag>
   <elementGuidId>5236041a-d466-44f1-a0fc-bea74bda1713</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;user_id&quot;,
      &quot;value&quot;: &quot;dfghjk&quot;
    },
    {
      &quot;name&quot;: &quot;goal_name&quot;,
      &quot;value&quot;: &quot;usa travelling&quot;
    },
    {
      &quot;name&quot;: &quot;goal_description&quot;,
      &quot;value&quot;: &quot;this goal is for a vacation in the united kingdom&quot;
    },
    {
      &quot;name&quot;: &quot;start_date&quot;,
      &quot;value&quot;: &quot;2018-12-12&quot;
    },
    {
      &quot;name&quot;: &quot;end_date&quot;,
      &quot;value&quot;: &quot;2019-12-12&quot;
    },
    {
      &quot;name&quot;: &quot;goal_amount&quot;,
      &quot;value&quot;: &quot;680000&quot;
    },
    {
      &quot;name&quot;: &quot;initial_deposit&quot;,
      &quot;value&quot;: &quot;120000&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://176.58.103.227:8087/api/create_goal</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
